use std::rc::Rc;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;

pub enum Asset {
    Mesh(Rc<Mesh>),
    Config(Config)
}