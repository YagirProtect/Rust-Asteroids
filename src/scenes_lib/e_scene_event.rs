use crate::classes::t_entity::Entity;

pub enum SceneEvent{
    None,
    SpawnEntity(Box<dyn Entity>),
    DestroyEntity(u32),
}