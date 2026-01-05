mod mesh_lib;
mod assetsdb_lib;
mod config_lib;
mod transform_lib;
mod classes;
mod render_lib;

use crate::config_lib::c_config::Config;
use std::time::Instant;
use minifb::{Window, WindowOptions};
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::test_object::TestObject;
use crate::mesh_lib::c_mesh::{Mesh, MeshLine};
use crate::render_lib::f_drawers::{draw_line, set_pixel};
use crate::render_lib::t_screen_data::Screen;
use crate::transform_lib::c_transform::Transform;

fn main() {

    let assets_db = AssetsDB::new();
    let config = match assets_db.get_any_asset_by_type::<Config>(){
        Some(c) => c,
        _ => {
            println!("Config cant be loaded");
            return;
        }
    };

    let mut screen = Screen::new(config.x(), config.y());

    let mut window = Window::new("Asteroids", screen.width(), screen.height(), WindowOptions::default()).unwrap();
    window.set_target_fps(240);




    let mut last = Instant::now();


    let render_object = TestObject::new(
        Transform::new(screen.center(), Vec2::new(1.0,1.0), 45.0),
        Mesh::new(vec![
            MeshLine::new(
                Vec2::new(0.0, 1.0),
                Vec2::new(0.0, -1.0),
            )
        ], false)
    );

    while window.is_open() {
        let mut delta_time = 0.0;
        {
            let now = Instant::now();
            delta_time = (now - last).as_secs_f32();
            last = now;

            if delta_time > 0.1 { delta_time = 0.1; }
        }
        screen.flush();

        let center = screen.center();

        draw_line(&mut screen, center.x + 460.0, center.y + 450.0, center.x-150.0, center.y-150.0);

        //LOGIC

        window
            .update_with_buffer(screen.get_buffer(), screen.width(), screen.height())
            .unwrap();
    }
}
