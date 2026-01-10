use crate::collisions_lib::e_col_layers::ColLayer;
use crate::mesh_lib::c_mesh::Mesh;
use std::rc::Rc;
use crate::transform_lib::c_transform::Transform;

pub trait Collide {
    fn can_collide(&self) -> bool {
        true
    }

    fn get_collision_layer(&self) -> ColLayer;

    fn get_collision_mesh(&self) -> Option<(Rc<Mesh>, &Transform)>{
        None
    }
    
    fn on_collision(&mut self, layer: ColLayer) {
        
    }
}