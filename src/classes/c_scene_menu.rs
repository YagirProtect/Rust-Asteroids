use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::scenes_lib::t_scene::Scene;
use egui::{Context, Frame, Rect, Ui};
use crate::render_lib::f_drawers::{ui_button, ui_get_card_rect, ui_header, ui_title_rect, ui_transparent_frame};
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
        let frame = ui_transparent_frame();

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui| {
                        ui_header(ui, "ASTEROIDS");

                        ui.add_space(ui.available_height() * 0.25);

                        ui.scope(|ui| {
                            ui.style_mut().text_styles.insert(
                                egui::TextStyle::Button,
                                egui::FontId::proportional(24.0),
                            );

                            ui.vertical_centered(|ui| {

                                for label in ["Play", "Leaderboard", "Credits", "Exit"] {
                                    if ui_button(ui, label) {
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
        let frame_bg = egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0))
            .stroke(egui::Stroke::NONE);

        egui::CentralPanel::default()
            .frame(frame_bg)
            .show(ctx, |ui| {
                let (avail, title_rect) = ui_title_rect(ui);

                ui.allocate_ui_at_rect(title_rect, |ui| {
                    ui_header(ui, "LEADERBOARD");
                });
                
                let (card_rect, card_frame) = ui_get_card_rect(avail);

                ui.allocate_ui_at_rect(card_rect, |ui| {
                    card_frame.show(ui, |ui| {

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

                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let leaderboard_data = self.web_client.get_leaderboard();

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

                                                ui.label(&e.name);

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

                        ui.horizontal_centered(|ui| {
                            if ui_button(ui, "Refresh") {
                                self.web_client.get_leaderboard_data();
                            }
                            ui.add_space(10.0);
                            if ui_button(ui, "Back") {
                                self.action = SceneSwitch::Switch(SceneId::Menu);
                            }
                        });
                    });
                });
            });
    }


    pub fn draw_credits_screen(&mut self, ctx: &Context) {
        let frame_bg = ui_transparent_frame();

        egui::CentralPanel::default()
            .frame(frame_bg)
            .show(ctx, |ui| {
                let (avail, title_rect) = ui_title_rect(ui);

                ui.allocate_ui_at_rect(title_rect, |ui| {
                    ui_header(ui, "CREDITS");
                });

                let (card_rect, card_frame) = ui_get_card_rect(avail);

                ui.allocate_ui_at_rect(card_rect, |ui| {
                    card_frame.show(ui, |ui| {

                        ui.label("Asteroids on RUST Lang");
                        ui.label("Created by Yagir");
                        ui.label("Repo: https://github.com/YagirProtect/Rust-Asteroids");

                        ui.horizontal_centered(|ui| {
                            if ui_button(ui, "Back") {
                                self.action = SceneSwitch::Switch(SceneId::Menu);
                            }
                        });
                    });
                });
            });
    }
}

