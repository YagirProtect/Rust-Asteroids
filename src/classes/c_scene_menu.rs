use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::t_scene::Scene;

#[derive(Default)]
pub struct MenuScene{
    entities: Vec<Box<dyn Entity>>,
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


}

