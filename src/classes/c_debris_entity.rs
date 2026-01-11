use std::rc::Rc;
use rand::Rng;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::bullet_entity::BulletEntity;
use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::collisions_lib::t_collision::Collide;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_event::SceneEvent::DestroyEntity;
use crate::transform_lib::c_transform::Transform;

pub struct DebrisEntity{
    id: u32,
    transform: Transform,
    mesh: Rc<Mesh>,
    speed: f32,


    rot_dir: f32,
    dir: Vec2<f32>,
    timer: f32,
}


impl Drawable for DebrisEntity {
    fn draw(&mut self, screen: &mut Screen) {
        
        self.draw_mesh(screen, &self.transform, &self.mesh);
    }
}

impl Collide for DebrisEntity {
    fn can_collide(&self) -> bool {
        false
    }
    fn get_collision_layer(&self) -> ColLayer {
        ColLayer::Player
    }
}

impl Entity for DebrisEntity {
    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        self.id
    }

    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {
        let vel = self.dir * self.speed;

        self.transform.add_rotation(self.rot_dir * delta_time);
        self.transform.set_velocity(vel);
        self.transform.update_position_by_vel(delta_time);

        self.timer += delta_time;

        if (self.timer > 1.5){

            return vec![
                DestroyEntity(self.id),
            ]
        }
        vec![]
    }

    fn get_position(&self) ->  &Vec2<f32> {
        self.transform.get_position()
    }
}

impl DebrisEntity {
    pub fn new(transform: Transform, mesh: Rc<Mesh>) -> DebrisEntity {

        let mut rng = rand::rng();

        let dir = transform.transform_dir_to_world(Vec2::new(1.0, 0.0));

        DebrisEntity {
            id: 0,
            transform: transform,
            mesh: mesh,
            speed: rng.random_range(150.0..350.0) as f32,
            timer: 0.0,
            dir: dir,
            rot_dir: rng.random_range(-1.0..1.0),
        }
    }
}