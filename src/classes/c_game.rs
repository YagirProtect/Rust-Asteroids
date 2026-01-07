use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::c_input::Input;
use crate::classes::c_scene_game::GameScene;
use crate::classes::c_scene_menu::MenuScene;
use crate::classes::c_scene_test::TestScene;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::scenes_lib::t_scene::Scene;

pub struct Game{
    screen: Screen,
    config: Config,
    assets_db: AssetsDB,

    scene: Box<dyn Scene>

}
impl Game {
    pub fn new() -> Self {

        let assets_db = AssetsDB::new();
        let config = assets_db.get_any_asset_by_type::<Config>().cloned().unwrap_or_default();
        let screen = Screen::new(config.x(), config.y());
        let scene = make_scene(SceneId::Test, &config, &screen);
        Self{
            assets_db,
            config,
            screen,
            scene: scene,
        }
    }



    pub fn update_game(&mut self, delta_time: f32, ctx: &egui::Context, input: &Input) -> bool {
        self.screen.flush();

        match self.scene.update(delta_time, input, &self.config) {
            SceneSwitch::None => {}
            SceneSwitch::Switch(new_scene_id) => {
                self.scene = make_scene(new_scene_id, &self.config, &self.screen);
            }
            SceneSwitch::Quit => {
                return false;
            }
        };
        self.scene.render(&mut self.screen);
        self.scene.ui(ctx);


        egui::Window::new("Debug").show(ctx, |ui| {
            ui.label(format!("dt: {:.4} sec", delta_time));
            ui.label(format!("present_mode: {:?}", self.scene.get_scene_name()));
        });


        return true;
    }

    
    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    pub fn get_assets_db(&self) -> &AssetsDB {
        &self.assets_db
    }

    pub fn get_current_scene(&self) -> &Box<dyn Scene> {
        &self.scene
    }
}

fn make_scene(scene_id: SceneId, config: &Config, screen: &Screen) -> Box<dyn Scene> {
    let mut value : Box<dyn Scene> = match scene_id {
        SceneId::Menu => Box::new(MenuScene::default()),
        SceneId::Game => Box::new(GameScene::default()),
        SceneId::Test => Box::new(TestScene::default())
    };
    
    value.create_scene(config, screen);
    
    return value;
}