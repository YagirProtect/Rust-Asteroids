use crate::classes::c_input::Input;
use crate::classes::c_scene_game::GameScene;
use crate::classes::c_scene_menu::MenuScene;
use crate::render_lib::t_screen_data::Screen;

pub enum SceneSwitch {
    None,
    MenuScene(MenuScene),
    GameScene(GameScene),
}

pub trait Scene {
    fn update(&mut self, dt: f32, input: &Input) -> SceneSwitch { SceneSwitch::None }
    fn render(&mut self, screen: &mut Screen);
    fn ui(&mut self, _ctx: &egui::Context) {}
}