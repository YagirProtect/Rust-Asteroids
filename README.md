# ASTEROIDS (Rust + pixels + egui)

A small **Asteroids-like** game written in Rust with classic **vector / wireframe** rendering.

The game draws into a custom CPU framebuffer (`Screen`), then presents it using **pixels** (wgpu). UI overlays (menu, HUD, game over, leaderboard) are built with **egui**. Sound effects are simple **procedural beeps** via **rodio**.


---

## Screenshots

<!-- Add your screenshots here -->
<p float="left">
  <img src="/screenshots/Screenshot_1.png" width="49%" />
  -
  <img src="/screenshots/Screenshot_2.png" width="49%" />
</p>

<p float="left">
  <img src="/screenshots/Screenshot_3.png" width="49%" />
  -
  <img src="/screenshots/Screenshot_4.png" width="49%" />
</p>

---

## Features

- **Vector / wireframe rendering** (line meshes)
- Custom **`Screen` framebuffer** layer (renderer-agnostic)
- Rendering to window via **pixels** (wgpu)
- UI with **egui**:
  - Main menu
  - In-game HUD (score + health)
  - Game over (nickname input)
  - Leaderboard screen
- Basic gameplay:
  - Player ship: thrust, rotation, shooting
  - Asteroid spawning and splitting
  - Enemy UFO + projectiles
  - Debris effects on explosions
- **Collision system** with per-layer collision filtering + line segment intersection
- **Asset database** (`AssetsDB`) that scans `data/` recursively and loads assets by file extension
- Online score submission + leaderboard fetch (optional; uses HTTP)

---

## Tech stack

Main crates used in this project:

- `pixels` (wgpu-backed framebuffer presentation)
- `winit` (window + events)
- `egui`, `egui-winit`, `egui-wgpu` (UI overlay)
- `vek` (math: `Vec2`)
- `rand` (spawns/variation)
- `rodio` (procedural beeps)
- `serde`, `serde_json` (assets as JSON)
- `reqwest` (optional: leaderboard HTTP)
- `image` (window icon loading)

---

## Project layout

- `src/`
  - `classes/` – game core (entities, scenes, app handler)
  - `render_lib/` – `Screen` + drawing helpers
  - `mesh_lib/` – mesh structs + (de)serialization
  - `assetsdb_lib/` – asset database + loaders
  - `collisions_lib/` – collision solver + segment intersection
  - `web_lib/` – HTTP client (submit score + fetch top)
- `data/` – runtime assets scanned by `AssetsDB`
  - `config.cfg` – window size config (JSON)
  - `icons/` – UI icons and window icon
  - `models/` *(recommended)* – `.mesh` vector models (ship, asteroids, UFO, etc.)

---

## Asset formats

### Mesh (`.mesh`)

Meshes are stored as JSON. A mesh is a list of 2D line segments (`MeshLine`), each with `from` and `to` points.

Example (simplified):

```json
{
  "name": "asteroid_01",
  "points": [
    { "from": {"x": -40.0, "y": 10.0}, "to": {"x": -10.0, "y": 40.0} },
    { "from": {"x": -10.0, "y": 40.0}, "to": {"x": 30.0, "y": 25.0} }
  ],
  "filled": false
}
```

> Tip: you can drop `.mesh` files anywhere under `data/` — `AssetsDB` will find them as long as the extension matches.

### Config (`.cfg`)

`data/config.cfg` is JSON (also `serde`-based). It defines the game’s logical window size.

---

## Build & run

### Requirements

- Rust toolchain (stable)
- A GPU/driver that supports wgpu (almost any modern integrated GPU is fine)

> Make sure the `data/` folder is available in your working directory when launching the game, because assets are loaded at runtime.

---

## Controls

- **A / D** – rotate left / right
- **W** – thrust
- **Space** – shoot

---

## Online leaderboard (optional)

The game can submit the final score and request the top list.

- Client code: `src/web_lib/c_web_client.rs`
- The endpoint is defined in `WEB_ADDRESS`.

Expected endpoints (default client behavior):

- **POST** `${WEB_ADDRESS}` – submit score as JSON
- **GET** `${WEB_ADDRESS}?action=top` – fetch Top 20

If you use your own backend, make sure the JSON fields match what your server expects (or adjust the client struct / serde renames accordingly).

---

## Notes

- Rendering is CPU-side: the whole frame is drawn into `Vec<u32>` and then uploaded/presented.
- The `Screen` abstraction exists so the renderer backend can be swapped later if desired.

---

## License

MIT
