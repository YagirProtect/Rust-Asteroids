use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::assetsdb_lib::t_file_writable::FileWritable;
use crate::classes::bullet_entity::BulletEntity;
use crate::classes::t_entity::Entity;
use crate::classes::player_entity::PlayerEntity;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::{Mesh, MeshLine};
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::t_scene::Scene;
use crate::transform_lib::c_transform::Transform;

#[derive(Default)]
pub struct TestScene{
    entities: Vec<Box<dyn Entity>>,
}

impl Scene for TestScene
{
    fn create_scene(&mut self, config: &Config, screen: &Screen, assets_db: &AssetsDB) {

        let mut player = PlayerEntity::new(
            Transform::new(
                screen.center(),
                Vec2::new(0.3, 0.3),
                0.0,
                config.size()
            ),
            assets_db.get_mesh_by_name("player").unwrap_or_default()
        );


        let mut bullet = BulletEntity::new(
            Transform::new(
                screen.center(),
                Vec2::new(0.3, 0.3),
                0.0,
                config.size()
            ),
            assets_db.get_mesh_by_name("bullet").unwrap_or_default(),
            0.0
        );


        self.add_entity(Box::new(player));
        self.add_entity(Box::new(bullet));
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

