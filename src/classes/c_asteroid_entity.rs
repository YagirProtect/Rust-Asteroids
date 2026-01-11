use std::rc::Rc;
use rand::Rng;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::collisions_lib::t_collision::Collide;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::transform_lib::c_transform::Transform;

pub struct AsteroidEntity {
    id: u32,
    transform: Transform,
    mesh: Rc<Mesh>,

    rotation: f32,
    is_need_destroy: bool
}




impl AsteroidEntity {
    pub fn new(transform: Transform, mesh: Rc<Mesh>) -> AsteroidEntity {
        
        
        let mut rnd = rand::rng();
        
        Self{
            id: 0,
            transform,
            mesh,
            rotation: rnd.random_range(-1.0..1.0)*0.5,
            is_need_destroy: false,
            
        }
    }

    pub fn set_velocity(&mut self, dir: Vec2<f32>) {
        self.transform.set_velocity(dir);
    }
}

impl Drawable for AsteroidEntity {
    fn draw(&mut self, screen: &mut Screen) {
        self.draw_mesh(screen, &self.transform, &self.mesh)
    }
}

impl Collide for AsteroidEntity {
    fn get_collision_layer(&self) -> ColLayer {
        return ColLayer::Asteroid;
    }

    fn get_collision_mesh(&self) -> Option<(Rc<Mesh>, &Transform)> {
        Some((self.mesh.clone(), &self.transform))
    }

    fn on_collision(&mut self, layer: ColLayer) {
        if (layer == ColLayer::BulletPlayer || layer == ColLayer::BulletEnemy || layer == ColLayer::Player || layer == ColLayer::Enemy) {
            self.is_need_destroy = true;
        }
    }
}

impl Entity for AsteroidEntity {
    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {
        let mut events = Vec::new();

        self.transform.add_rotation(self.rotation * delta_time);
        self.transform.update_position_by_vel(delta_time);

        if (self.is_need_destroy) {
            events.push(SceneEvent::DemolishAsteroid{
                pos: *self.transform.get_position(),
                scale: self.transform.get_scale().magnitude(),
                id: self.id,
            });
            events.push(SceneEvent::DestroyEntity(self.id));
        }
        events
    }
    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        self.id
    }

    fn get_position(&self) ->  &Vec2<f32> {
        self.transform.get_position()
    }
}