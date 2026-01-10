<?php
declare(strict_types=1);

header('Content-Type: application/json; charset=utf-8');

$dbFile = __DIR__ . DIRECTORY_SEPARATOR . 'data.txt';

/**
 * Safe nickname normalization for the file-based DB format:
 * - removes \r \n \t
 * - forbids ":" (to avoid breaking parsing)
 * - limits length
 * - keeps only letters/digits/underscore/space (you can relax this)
 */
function normalize_name(string $name): string {
    $name = trim($name);
    $name = str_replace(["\r", "\n", "\t"], " ", $name);
    $name = preg_replace('/\s+/', ' ', $name) ?? $name;
    $name = str_replace(':', '_', $name);

    $name = preg_replace('/[^\p{L}\p{N}_ ]/u', '', $name) ?? $name;

    // length 2..16
    $name = mb_substr($name, 0, 16, 'UTF-8');
    return $name;
}

function read_all_from_handle($fh): string {
    rewind($fh);
    $data = stream_get_contents($fh);
    return $data === false ? '' : $data;
}

function parse_db_text(string $text): array {
    $scores = []; // name => int
    $lines = preg_split("/\r\n|\n|\r/", $text) ?: [];
    foreach ($lines as $line) {
        $line = trim($line);
        if ($line === '') continue;

        $parts = explode(':', $line, 2);
        if (count($parts) !== 2) continue;

        $name = trim($parts[0]);
        $val  = trim($parts[1]);

        if ($name === '') continue;
        if (!is_numeric($val)) continue;

        $scores[$name] = (int)$val;
    }
    return $scores;
}

function db_to_text(array $scores): string {
    // Write to file as "name: value" line by line.
    // No need to sort here; sorting is only needed for top results.
    $out = '';
    foreach ($scores as $name => $score) {
        // Just in case: strip newlines and ":" from the name
        $safeName = str_replace(["\r","\n",":"], ['','','_'], (string)$name);
        $out .= $safeName . ': ' . (int)$score . "\n";
    }
    return $out;
}

function json_error(int $code, string $msg): void {
    http_response_code($code);
    echo json_encode(['ok' => false, 'error' => $msg], JSON_UNESCAPED_UNICODE);
    exit;
}

function json_ok(array $payload): void {
    echo json_encode(array_merge(['ok' => true], $payload), JSON_UNESCAPED_UNICODE);
    exit;
}

/** GET top 20 */
function handle_top(string $dbFile): void {
    if (!file_exists($dbFile)) {
        json_ok(['top' => []]);
    }

    $fh = fopen($dbFile, 'r');
    if (!$fh) json_error(500, 'Cannot open data.txt');

    // Shared lock for reading
    flock($fh, LOCK_SH);
    $text = stream_get_contents($fh);
    flock($fh, LOCK_UN);
    fclose($fh);

    $scores = parse_db_text($text ?: '');

    // Sort by score desc
    uasort($scores, function($a, $b) {
        if ($a === $b) return 0;
        return ($a > $b) ? -1 : 1;
    });

    // Convert to list of objects
    $top = [];
    foreach ($scores as $name => $score) {
        $top[] = ['name' => $name, 'score' => (int)$score];
        if (count($top) >= 20) break;
    }

    json_ok(['top' => $top]);
}

/** POST submit */
function handle_submit(string $dbFile): void {
    // Read input: form-data/x-www-form-urlencoded or JSON
    $name = '';
    $scoreRaw = null;

    $contentType = $_SERVER['CONTENT_TYPE'] ?? '';
    if (stripos($contentType, 'application/json') !== false) {
        $raw = file_get_contents('php://input');
        $data = json_decode($raw ?: '', true);
        if (!is_array($data)) json_error(400, 'Bad JSON');
        $name = (string)($data['name'] ?? '');
        $scoreRaw = $data['score'] ?? null;
    } else {
        $name = (string)($_POST['name'] ?? '');
        $scoreRaw = $_POST['score'] ?? null;
    }

    $name = normalize_name($name);
    if (mb_strlen($name, 'UTF-8') < 2) json_error(400, 'Name must be at least 2 characters');

    if ($scoreRaw === null || $scoreRaw === '') json_error(400, 'Missing score');
    if (!is_numeric($scoreRaw)) json_error(400, 'Score must be a number');

    $score = (int)$scoreRaw;
    if ($score < 0) $score = 0;

    // Open/create file and take an exclusive lock
    $fh = fopen($dbFile, 'c+'); // create if not exists
    if (!$fh) json_error(500, 'Cannot open data.txt');

    flock($fh, LOCK_EX);

    $text = read_all_from_handle($fh);
    $scores = parse_db_text($text);

    $prev = $scores[$name] ?? null;
    $updated = false;

    if ($prev === null || $score > (int)$prev) {
        $scores[$name] = $score;
        $updated = true;
    }

    // Write back
    $out = db_to_text($scores);
    rewind($fh);
    ftruncate($fh, 0);
    fwrite($fh, $out);

    flock($fh, LOCK_UN);
    fclose($fh);

    json_ok([
        'name' => $name,
        'score' => $scores[$name],
        'updated' => $updated,
        'previous' => $prev,
    ]);
}

// ---- ROUTER ----
$method = $_SERVER['REQUEST_METHOD'] ?? 'GET';
$action = $_GET['action'] ?? '';

if ($method === 'GET') {
    // /index.php?action=top
    // or just /index.php
    if ($action === 'top' || $action === '' ) {
        handle_top($dbFile);
    }
    json_error(404, 'Unknown action');
}

if ($method === 'POST') {
    // /index.php (POST)
    handle_submit($dbFile);
}

json_error(405, 'Method not allowed');
