use crate::mesh_lib::c_mesh::Mesh;
use crate::transform_lib::c_transform::Transform;

pub struct TestObject{
    transform: Transform,
    mesh: Mesh
}


impl TestObject{
    pub fn new(transform: Transform, mesh: Mesh) -> TestObject{
        TestObject{transform: transform, mesh: mesh}
    }
}