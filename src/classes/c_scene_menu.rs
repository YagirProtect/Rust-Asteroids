use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::scenes_lib::t_scene::Scene;
use egui::Context;

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

            }
            MenuTab::LeaderboardScreen => {

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

                                for label in ["Play", "Scores", "Credits", "Exit"] {
                                    if ui.add(egui::Button::new(label).min_size(btn_size)).clicked() {
                                        match label {
                                            "Exit" => {
                                                self.action = SceneSwitch::Quit;
                                            }
                                            "Play" => {
                                                self.action = SceneSwitch::Switch(SceneId::Game);
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
}

