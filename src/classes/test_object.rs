use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::t_drawable::Drawable;
use crate::render_lib::t_screen_data::Screen;
use crate::transform_lib::c_transform::Transform;

pub struct TestObject{
    transform: Transform,
    mesh: Mesh
}


impl TestObject{
    pub fn new(transform: Transform, mesh: Mesh) -> TestObject{
        TestObject{transform: transform, mesh: mesh}
    }
    
    pub fn get_transform(&mut self) -> &mut Transform{
        &mut self.transform
    }
}


impl Drawable for TestObject{
    fn draw(&mut self, screen: &mut Screen){
        self.draw_mesh(screen, &self.transform, &self.mesh);
    }
}