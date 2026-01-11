use std::rc::Rc;
use egui::{vec2, Align, Context, Layout};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::c_asteroid_entity::AsteroidEntity;
use crate::classes::c_player_entity::PlayerEntity;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::t_scene::Scene;
use crate::transform_lib::c_transform::Transform;
use rand::seq::IteratorRandom;
use vek::Vec2;
use crate::classes::c_enemy_entity::EnemyEntity;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::f_drawers::{ui_button, ui_draw_icon, ui_get_card_rect, ui_title_rect, ui_transparent_frame};
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::sprite_lib::c_sprite::SpriteTex;
use crate::web_lib::c_web_client::WebClient;



#[derive(Default)]
pub struct TestScene {
    entities: Vec<Box<dyn Entity>>,
}

impl Scene for TestScene
{
    fn create_scene(&mut self, config: &Config, screen: &Screen, assets_db: &AssetsDB) {
        let mut enemy = EnemyEntity::new(
            Transform::new(
                screen.center(),
                Vec2::new(0.7, 0.7),
                0.0,
                config.size()
            ),
            assets_db.get_mesh_by_name("ufo_01").unwrap_or_default()
        );



        self.add_entity(Box::new(enemy));
    }
    fn get_scene_name(&self) -> String{
        String::from("TestScene")
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }


}



