use std::rc::Rc;
use rand::Rng;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::bullet_entity::BulletEntity;
use crate::classes::c_input::Input;
use crate::classes::c_player_entity::PlayerEntity;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::collisions_lib::t_collision::Collide;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_event::SceneEvent::SpawnEntity;
use crate::transform_lib::c_transform::Transform;

pub struct EnemyEntity{
    id: u32,
    transform : Transform,
    mesh: Rc<Mesh>,
    destroy: bool,

    x_dir: i8,

    global_time: f32,
    speed: f32,

    shoot_time: f32,
    shoot_cooldown: f32
}

impl EnemyEntity {

    pub fn new(transform: Transform, mesh: Rc<Mesh>) -> EnemyEntity {

        let mut rnd = rand::rng();

        let mut dir = 0;

        while dir == 0 {
            dir = rnd.random_range(-1..1);
        }

        EnemyEntity {
            id: 0,
            transform: transform,
            mesh: mesh,
            destroy: false,
            x_dir: dir,
            global_time: 0.0,
            speed: 250.0,
            shoot_time: 0.0,
            shoot_cooldown: 0.25,
        }
    }
    fn spawn_bullet(&mut self, events: &mut Vec<SceneEvent>, assets_db: &AssetsDB, config: &Config) {
        let mesh = assets_db.get_mesh_by_name("bullet");
        let spawn_point = self.transform.transform_point_to_world(Vec2::new(50.0, 0.0));
        let mut rnd = rand::rng();

        let rot = rnd.random_range(-360.0..360.0);

        let bullet = BulletEntity::new(
            Transform::new(
                spawn_point,
                Vec2::new(0.3, 0.3),
                rot,
                config.size()
            ),
            mesh.unwrap_or_default(),
            500.0,
            ColLayer::BulletEnemy
        );

        events.push(SpawnEntity(Box::new(bullet)));
    }
}

impl Drawable for EnemyEntity {
    fn draw(&mut self, screen: &mut Screen) {
        self.draw_mesh(screen, &self.transform, &self.mesh);
    }
}

impl Collide for EnemyEntity {
    fn get_collision_layer(&self) -> ColLayer {
        ColLayer::Enemy
    }
    fn on_collision(&mut self, layer: ColLayer) {
        if (layer == ColLayer::BulletPlayer || layer == ColLayer::Asteroid) {
            self.destroy = true;
        }
    }
    fn get_collision_mesh(&self) -> Option<(Rc<Mesh>, &Transform)> {
        return Some((self.mesh.clone(), &self.transform))
    }
}

impl Entity for EnemyEntity {
    fn update(&mut self, delta_time: f32, input: &Input, config: &Config, assets_db: &AssetsDB) -> Vec<SceneEvent> {

        let mut events: Vec<SceneEvent> = Vec::new();

        if (self.destroy) {
            events.push(SceneEvent::DestroyEntity(self.id));
            events.push(SceneEvent::SpawnDebris(*self.transform.get_position()));
        }

        self.global_time += delta_time * 5.0;

        self.shoot_time += delta_time;

        let mut dir = Vec2::new(self.x_dir as f32, self.global_time.sin()).normalized();

        self.transform.set_velocity(dir * self.speed);


        self.transform.update_position_by_vel(delta_time);


        if (self.shoot_time >= self.shoot_cooldown){
            self.spawn_bullet(&mut events, assets_db, config);
            self.shoot_time = 0.0;
        }

        events
    }

    fn set_entity_id(&mut self, entity_id: u32) {
        self.id = entity_id;
    }

    fn get_entity_id(&self) -> u32 {
        self.id
    }

    fn get_position(&self) -> &Vec2<f32> {
        self.transform.get_position()
    }
}