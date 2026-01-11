use vek::Vec2;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;

pub enum SceneEvent{
    None,
    SpawnEntity(Box<dyn Entity>),
    DestroyEntity(u32),
    Collision { a: u32, b: u32 },
    DemolishAsteroid{pos: Vec2<f32>, scale: f32, id: u32},
    PlayerDeath,
    SpawnDebris(Vec2<f32>),
    Shoot(ColLayer),
}