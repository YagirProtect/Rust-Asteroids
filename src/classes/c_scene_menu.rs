use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::scenes_lib::t_scene::Scene;
use egui::Context;
use crate::web_lib::c_web_client::{LeaderboardState, WebClient};

#[derive(Default)]
enum MenuTab{
    #[default]
    MainScreen,
    CreditsScreen,
    LeaderboardScreen
}

#[derive(Default)]
pub struct MenuScene{
    entities: Vec<Box<dyn Entity>>,

    web_client: WebClient,
    tab: MenuTab,
    action: SceneSwitch
}

impl Scene for MenuScene
{
    fn create_scene(&mut self, config: &Config, screen: &Screen, assets_db: &AssetsDB) {
    }
    
    fn get_scene_name(&self) -> String{
        String::from("MenuScene")
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }


    fn ui(&mut self, ctx: &Context) -> SceneSwitch {

        self.action = SceneSwitch::None;

        match self.tab {
            MenuTab::MainScreen => {
                self.draw_main_screen(ctx);
            }
            MenuTab::CreditsScreen => {
                self.draw_credits_screen(ctx);
            }
            MenuTab::LeaderboardScreen => {
                self.draw_leaderboard_screen(ctx);
            }
        }



        self.action
    }

}

impl MenuScene {
    fn draw_main_screen(&mut self, ctx: &Context) {
        let frame = egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0))
            .stroke(egui::Stroke::NONE);

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui.add_space(ui.available_height() * 0.15);
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("ASTEROIDS")
                                    .size(80.0)
                                    .strong(),
                            );
                        });

                        ui.add_space(ui.available_height() * 0.25);

                        ui.scope(|ui| {
                            ui.style_mut().text_styles.insert(
                                egui::TextStyle::Button,
                                egui::FontId::proportional(24.0),
                            );

                            ui.vertical_centered(|ui| {
                                let btn_size = egui::vec2(260.0, 30.0);

                                for label in ["Play", "Leaderboard", "Credits", "Exit"] {
                                    if ui.add(egui::Button::new(label).min_size(btn_size)).clicked() {
                                        match label {
                                            "Exit" => {
                                                self.action = SceneSwitch::Quit;
                                            }
                                            "Play" => {
                                                self.action = SceneSwitch::Switch(SceneId::Game);
                                            },
                                            "Leaderboard" => {
                                                self.web_client.get_leaderboard_data();
                                                self.tab = MenuTab::LeaderboardScreen;
                                            }
                                            "Credits" => {
                                                self.tab = MenuTab::CreditsScreen;
                                            }
                                            _ => {}
                                        };
                                    }
                                    ui.add_space(8.0);
                                }
                            });
                        });
                    },
                );
            });
    }


    pub fn draw_leaderboard_screen(&mut self, ctx: &Context) {
        // фон
        let frame_bg = egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0))
            .stroke(egui::Stroke::NONE);

        egui::CentralPanel::default()
            .frame(frame_bg)
            .show(ctx, |ui| {
                let avail = ui.max_rect();

                // Заголовок сверху
                let title_h = 110.0;
                let title_rect = egui::Rect::from_min_size(
                    egui::pos2(avail.left(), avail.top() + avail.height() * 0.08),
                    egui::vec2(avail.width(), title_h),
                );

                ui.allocate_ui_at_rect(title_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("LEADERBOARD")
                                .size(80.0)
                                .strong(),
                        );
                    });
                });

                // "Окошко" под заголовком
                let card_w = (avail.width() * 0.70).min(900.0).max(520.0);
                let card_h = (avail.height() * 0.62).min(650.0).max(280.0);

                let card_center = egui::pos2(avail.center().x, avail.top() + avail.height() * 0.55);
                let mut card_rect = egui::Rect::from_center_size(card_center, egui::vec2(card_w, card_h));

                // snap к целым пикселям, чтобы не "трясло"
                card_rect.min.x = card_rect.min.x.round();
                card_rect.min.y = card_rect.min.y.round();
                card_rect.max.x = card_rect.max.x.round();
                card_rect.max.y = card_rect.max.y.round();

                let card_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 20, 220))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 80, 80)))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(16.0));

                ui.allocate_ui_at_rect(card_rect, |ui| {
                    card_frame.show(ui, |ui| {
                        // Шапка таблицы
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("#").size(18.0).strong());
                            ui.add_space(12.0);
                            ui.label(egui::RichText::new("NAME").size(18.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("SCORE").size(18.0).strong());
                            });
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Контент: тут ты вставишь реальные данные
                        // Предположим, что у тебя есть self.leaderboard: Vec<Entry { name, score }>
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                // пример: заменишь на свой источник
                                let leaderboard_data = self.web_client.get_leaderboard(); // <- сделай метод/поле как удобно


                                match leaderboard_data {
                                    LeaderboardState::Idle => {}
                                    LeaderboardState::Loading => {
                                        ui.label("loading...");
                                        self.web_client.poll_network();
                                    }
                                    LeaderboardState::Ready(list) => {
                                        for (i, e) in list.iter().enumerate() {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("{:>2}", i + 1));
                                                ui.add_space(12.0);

                                                // имя слева
                                                ui.label(&e.name);

                                                // score справа
                                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                    ui.label(format!("{}", e.score));
                                                });
                                            });
                                            ui.add_space(4.0);
                                        }

                                        if list.is_empty() {
                                            ui.add_space(10.0);
                                            ui.label("No data. Press Refresh.");
                                        }
                                    }
                                    LeaderboardState::Error(error) => {
                                        ui.label(format!("{}", error));
                                    }
                                }


                            });

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(12.0);

                        // Кнопки
                        let btn = egui::vec2(140.0, 36.0);
                        ui.horizontal_centered(|ui| {
                            if ui.add_sized(btn, egui::Button::new("Refresh")).clicked() {
                                self.web_client.get_leaderboard_data();
                            }
                            ui.add_space(10.0);
                            if ui.add_sized(btn, egui::Button::new("Back")).clicked() {
                                self.action = SceneSwitch::Switch(SceneId::Menu);
                            }
                        });
                    });
                });
            });
    }


    pub fn draw_credits_screen(&mut self, ctx: &Context) {
        // фон
        let frame_bg = egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0))
            .stroke(egui::Stroke::NONE);

        egui::CentralPanel::default()
            .frame(frame_bg)
            .show(ctx, |ui| {
                let avail = ui.max_rect();

                // Заголовок сверху
                let title_h = 110.0;
                let title_rect = egui::Rect::from_min_size(
                    egui::pos2(avail.left(), avail.top() + avail.height() * 0.08),
                    egui::vec2(avail.width(), title_h),
                );

                ui.allocate_ui_at_rect(title_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("CREDITS")
                                .size(80.0)
                                .strong(),
                        );
                    });
                });

                // "Окошко" под заголовком
                let card_w = (avail.width() * 0.70).min(900.0).max(520.0);
                let card_h = (avail.height() * 0.62).min(650.0).max(280.0);

                let card_center = egui::pos2(avail.center().x, avail.top() + avail.height() * 0.55);
                let mut card_rect = egui::Rect::from_center_size(card_center, egui::vec2(card_w, card_h));

                // snap к целым пикселям, чтобы не "трясло"
                card_rect.min.x = card_rect.min.x.round();
                card_rect.min.y = card_rect.min.y.round();
                card_rect.max.x = card_rect.max.x.round();
                card_rect.max.y = card_rect.max.y.round();

                let card_frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(20, 20, 20, 220))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(80, 80, 80)))
                    .rounding(egui::Rounding::same(12.0))
                    .inner_margin(egui::Margin::same(16.0));

                ui.allocate_ui_at_rect(card_rect, |ui| {
                    card_frame.show(ui, |ui| {

                        ui.label("Asteroids on RUST Lang");
                        ui.label("Created by Yagir");
                        ui.label("Repo: https://github.com/YagirProtect/Rust-Asteroids");

                        let btn = egui::vec2(140.0, 36.0);
                        ui.horizontal_centered(|ui| {
                            if ui.add_sized(btn, egui::Button::new("Back")).clicked() {
                                self.action = SceneSwitch::Switch(SceneId::Menu);
                            }
                        });
                    });
                });
            });
    }
}

