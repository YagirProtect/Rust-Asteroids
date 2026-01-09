use std::rc::Rc;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::bullet_entity::BulletEntity;
use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
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
            shoot_time: 0.0
        }
    }
}

impl Drawable for PlayerEntity {
    fn draw(&mut self, screen: &mut Screen) {
        self.draw_mesh(screen, &self.transform, &self.mesh);
    }
}

impl Entity for PlayerEntity {
    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        return self.id;
    }

    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {

        let mut events = vec![];

        self.transform.add_rotation(delta_time * 5.0 * input.get_axis_hor());

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
            self.transform.get_velocity().magnitude()
        );


        events.push(SpawnEntity(Box::new(bullet)));
    }
}

