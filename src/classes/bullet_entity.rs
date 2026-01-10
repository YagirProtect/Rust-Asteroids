use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::transform_lib::c_transform::Transform;
use std::rc::Rc;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::collisions_lib::t_collision::Collide;
use crate::scenes_lib::e_scene_event::SceneEvent::DestroyEntity;

pub struct BulletEntity {
    id: u32,
    transform: Transform,
    mesh: Rc<Mesh>,
    speed: f32,

    timer: f32
}

impl Drawable for BulletEntity {
    fn draw(&mut self, screen: &mut Screen) {
        self.draw_mesh(screen, &self.transform, &self.mesh);
    }
}

impl Collide for BulletEntity {
    fn get_collision_layer(&self) -> ColLayer {
        ColLayer::BulletPlayer
    }

    fn get_collision_mesh(&self) -> Option<(Rc<Mesh>, &Transform)> {
        Some((self.mesh.clone(), &self.transform))
    }

    fn on_collision(&mut self, layer: ColLayer) {
        self.timer = 1000.0;
    }
}

impl Entity for BulletEntity {
    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        self.id
    }

    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {
        let vel = self.transform.transform_dir_to_world(Vec2::new(1.0, 0.0)) * self.speed;
        self.transform.set_velocity(vel);
        self.transform.update_position_by_vel(delta_time);

        self.timer += delta_time;

        if (self.timer > 0.5){
            
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

impl BulletEntity {
    pub fn new(transform: Transform, mesh: Rc<Mesh>, start_speed: f32) -> BulletEntity {


        BulletEntity {
            id: 0,
            transform: transform,
            mesh: mesh,
            speed: start_speed + 2000.0,
            timer: 0.0
        }
    }
}