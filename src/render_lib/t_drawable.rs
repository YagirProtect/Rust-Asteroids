use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::f_drawers::{draw_line_thick};
use crate::render_lib::t_screen_data::Screen;
use crate::transform_lib::c_transform::Transform;

pub trait Drawable{

    fn enabled_renderer(&self) -> bool {true}
    
    fn draw(&mut self, screen: &mut Screen) {}
    
    fn draw_mesh(&self, screen: &mut Screen, transform: &Transform, mesh: &Mesh){

        if (!mesh.is_filled()) {
            mesh.get_lines().iter().for_each(|point|{
                let start = transform.transform_point_to_world(point.start);
                let end = transform.transform_point_to_world(point.end);

                draw_line_thick(screen, start.x, start.y, end.x, end.y, 3, u32::MAX);
            })
        }
    }
}