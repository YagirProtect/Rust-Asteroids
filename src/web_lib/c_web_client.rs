use std::cmp::PartialEq;
use std::sync::mpsc;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct WebClient {
    nickname: String,
    leaderboard: LeaderboardState,
    lb_rx: Option<mpsc::Receiver<LeaderboardState>>,
}

#[derive(Clone, Default)]
pub enum LeaderboardState {
    #[default]
    Idle,
    Loading,
    Ready(Vec<SendData>),
    Error(String),
}
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct  SendData{
    pub name: String,
    pub score: u32,
}

#[derive(Deserialize)]
struct TopResponse {
    ok: bool,
    #[serde(default)]
    top: Vec<SendData>,
    #[serde(default)]
    error: Option<String>,
}


pub static WEB_ADDRESS: &str = "https://stats.yagir.xyz/asteroids/";

impl WebClient {
    pub fn change_nickname(&mut self, nickname: String) {
        self.nickname = nickname;

        self.nickname.retain(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == ' ');
        if self.nickname.len() > 16 {
            self.nickname.truncate(16);
        }
    }

    pub fn is_available_name(&self) -> bool {
        self.nickname.len() >= 2
    }

    pub fn get_nickname(&self) -> String {
        self.nickname.clone()
    }
    pub fn send_web_data(&self, scores: u32) {
        let data = SendData {
            score: scores,
            name: self.nickname.clone()
        };

        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let res = client
                .post(WEB_ADDRESS)
                .json(&data)
                .send();

            match res {
                Ok(r) => {
                    let status = r.status();
                    let body = r.text().unwrap_or_default();
                    eprintln!("HTTP {status}\n{body}");
                }
                Err(e) => eprintln!("request error: {e}"),
            }
        });
    }

    pub fn poll_network(&mut self) {
        if let Some(rx) = &self.lb_rx {
            if let Ok(new_state) = rx.try_recv() {
                self.leaderboard = new_state;
                self.lb_rx = None; // запрос завершён
            }
        }
    }

    pub fn get_leaderboard(&self) -> LeaderboardState {
        return self.leaderboard.clone();
    }

    pub fn get_leaderboard_data(&mut self){
        if matches!(self.leaderboard, LeaderboardState::Loading) {
            return;
        }

        self.leaderboard = LeaderboardState::Loading;

        let (sender, receiver) = mpsc::channel();
        self.lb_rx = Some(receiver);

        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(5))
                .build();

            let client = match client {
                Ok(c) => c,
                Err(e) => {
                    let _ = sender.send(LeaderboardState::Error(format!("Client build: {e}")));
                    return;
                }
            };

            let res = client
                .get(WEB_ADDRESS)
                .query(&[("action", "top")])
                .send();

            let state = match res {
                Ok(r) => {
                    let status = r.status();
                    let text = r.text().unwrap_or_default();

                    if !status.is_success() {
                        LeaderboardState::Error(format!("HTTP {status}: {text}"))
                    } else {
                        match serde_json::from_str::<TopResponse>(&text) {
                            Ok(parsed) if parsed.ok => LeaderboardState::Ready(parsed.top),
                            Ok(parsed) => LeaderboardState::Error(parsed.error.unwrap_or("Server error".into())),
                            Err(e) => LeaderboardState::Error(format!("Bad JSON: {e}. Body: {text}")),
                        }
                    }
                }
                Err(e) => LeaderboardState::Error(format!("Request error: {e}")),
            };

            let _ = sender.send(state);
        });
    }
}