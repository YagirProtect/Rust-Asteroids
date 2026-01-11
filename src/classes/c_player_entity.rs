use std::rc::Rc;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::bullet_entity::BulletEntity;
use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::collisions_lib::t_collision::Collide;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::{Mesh, MeshLine};
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_event::SceneEvent::SpawnEntity;
use crate::transform_lib::c_transform::Transform;

pub struct PlayerEntity {
    id: u32,
    transform: Transform,
    mesh: Rc<Mesh>,

    max_speed: f32,
    acceleration_speed: f32,
    decceleration_speed: f32,
    shoot_cooldown: f32,

    aceel: f32,

    shoot_time: f32,
    god_mode_time: f32,

    global_time: f32,

    is_hitted: bool,
    is_god_mode: bool,

}


impl PlayerEntity {
    pub fn new(transform: Transform, mesh: Rc<Mesh>) -> PlayerEntity {
        PlayerEntity {
            id: 0,
            transform: transform,
            mesh: mesh,
            max_speed: 650.0,
            acceleration_speed: 250.0,
            decceleration_speed: 1.0,
            aceel: 0.0,
            shoot_cooldown: 0.25,
            shoot_time: 0.0,
            global_time: 0.0,
            god_mode_time: 0.0,
            is_god_mode: false,
            is_hitted: false,
        }
    }

    fn create_thruster_mesh(dist: f32, wobble: f32) -> Mesh {
        let mut procedurialTrusterMesh = Mesh::new(
            "thruster".to_string(),
            vec![
                MeshLine::new(Vec2::new(-80.0, 30.0), Vec2::new(dist-80.0, wobble)),
                MeshLine::new(Vec2::new(-80.0, -30.0), Vec2::new(dist-80.0, wobble)),
            ],
            false
        );
        procedurialTrusterMesh
    }
}

impl Drawable for PlayerEntity {
    fn draw(&mut self, screen: &mut Screen) {


        if (self.is_god_mode){
            let sin = (self.global_time * 20.0).sin();
            if (sin > 0.5){
                return;
            }
        }

        self.draw_mesh(screen, &self.transform, &self.mesh);

        let truster = Self::create_thruster_mesh(-self.transform.get_velocity().magnitude() / 5.0, (self.global_time * 20.0).sin() * 5.0);

        self.draw_mesh(screen, &self.transform, &truster);
    }
}

impl Collide for PlayerEntity {
    fn get_collision_layer(&self) -> ColLayer {
        return ColLayer::Player;
    }

    fn get_collision_mesh(&self) -> Option<(Rc<Mesh>, &Transform)> {
        Some((self.mesh.clone(), &self.transform))
    }

    fn on_collision(&mut self, layer: ColLayer) {
        if (self.is_god_mode) {return};
        if (layer == ColLayer::BulletEnemy || layer == ColLayer::Asteroid || layer == ColLayer::Enemy){
            self.is_hitted = true;
        }
    }
}

impl Entity for PlayerEntity {
    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        return self.id;
    }
    fn get_position(&self) ->  &Vec2<f32> { return self.transform.get_position() }

    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {

        let mut events = vec![];

        if (self.is_hitted){
            self.is_hitted = false;

            self.transform.set_velocity(Vec2::zero());
            self.transform.update_position_warp(Vec2::new(
                (config.x()/2) as f32,
                (config.y()/2) as f32
            ));
            self.god_mode_time = 0.0;
            self.is_god_mode = true;

            events.push(SceneEvent::PlayerDeath);
        }

        if (self.is_god_mode){
            self.god_mode_time += delta_time;
            if (self.god_mode_time > 5.0){
                self.is_god_mode = false;
            }
        }


        self.transform.add_rotation(delta_time * 5.0 * input.get_axis_hor());

        self.global_time += delta_time;

        if input.get_axis_ver() > 0.5 {
            self.aceel = (self.aceel + (5.0 * delta_time)).min(1.0);

            let mut accel_val = self.aceel * self.acceleration_speed; // units/sec^2
            let forward = self
                .transform
                .transform_dir_to_world(Vec2::new(1.0, 0.0))
                .normalized();

            let mut v = self.transform.get_velocity().clone();


            let fly_backward = v.magnitude() > 0.0 && v.dot(forward) < 0.0;

            if (fly_backward){
                accel_val *= 2.0;
            }


            v += forward * accel_val * delta_time;



            let speed = v.magnitude();
            if speed > self.max_speed {
                v = v / speed * self.max_speed;
            }

            self.transform.set_velocity(v);

        } else {
            let vel = self.transform.get_velocity().clone();
            let t = (self.decceleration_speed * delta_time).clamp(0.0, 1.0);
            let v = Vec2::lerp(vel, Vec2::zero(), t);
            self.transform.set_velocity(v);
        }




        self.transform.update_position_by_vel(delta_time);


        self.shoot_time += delta_time;

        if (input.get_fire()) {
            if (self.shoot_time >= self.shoot_cooldown) {
                self.spawn_bullet(&mut events, assets_db, config);

                self.shoot_time = 0.0
            }
        }


        events
    }
}

impl PlayerEntity {
    fn spawn_bullet(&mut self, events: &mut Vec<SceneEvent>, assets_db: &AssetsDB, config: &Config) {
        let mesh = assets_db.get_mesh_by_name("bullet");
        let spawn_point = self.transform.transform_point_to_world(Vec2::new(50.0, 0.0));

        let bullet = BulletEntity::new(
            Transform::new(
                spawn_point,
                Vec2::new(0.3, 0.3),
                self.transform.get_rotation(),
                config.size()
            ),
            mesh.unwrap_or_default(),
            self.transform.get_velocity().magnitude(),
            ColLayer::BulletPlayer
        );


        events.push(SpawnEntity(Box::new(bullet)));
    }
}

