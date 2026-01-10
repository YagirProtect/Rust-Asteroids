use std::rc::Rc;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;
use crate::sprite_lib::c_sprite::SpriteTex;

pub enum Asset {
    Mesh(Rc<Mesh>),
    Config(Config),
    Sprite(Rc<SpriteTex>),
}