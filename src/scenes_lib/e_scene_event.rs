use vek::Vec2;
use crate::classes::t_entity::Entity;

pub enum SceneEvent{
    None,
    SpawnEntity(Box<dyn Entity>),
    DestroyEntity(u32),
    Collision { a: u32, b: u32 },
    DemolishAsteroid{pos: Vec2<f32>, scale: f32},
    PlayerDeath
}