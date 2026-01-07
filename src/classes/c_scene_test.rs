use vek::Vec2;
use crate::classes::t_entity::Entity;
use crate::classes::test_object::PlayerEntity;
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
    fn create_scene(&mut self, config: &Config, screen: &Screen) {
        let mut player = PlayerEntity::new(
            Transform::new(
                screen.center(),
                Vec2::new(0.3, 0.3), 
                0.0,
                config.size()
            ),
            Mesh::new(vec![
                MeshLine::new(
                    Vec2::new(-80.0, 50.0),
                    Vec2::new(-80.0, -50.0),
                ),
                MeshLine::new(
                    Vec2::new(-80.0, 50.0),
                    Vec2::new(80.0, 0.0),
                ),
                MeshLine::new(
                    Vec2::new(-80.00, -50.0),
                    Vec2::new(80.0, 0.0),
                ),
            ], false)
        );


        self.add_entity(Box::new(player));
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

