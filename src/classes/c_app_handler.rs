use crate::classes::c_game::Game;

use std::{error::Error, time::Instant};
use std::path::Path;
use std::sync::Arc;
use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State as EguiWinitState;
use pixels::{wgpu, Pixels, SurfaceTexture};
use vek::Vec2;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::keyboard::PhysicalKey;
use winit::window::{Icon, Window, WindowButtons};
use crate::classes::c_input::Input;

#[derive(Copy, Clone, Default)]
pub struct AppHandler {

}



impl AppHandler {
    pub fn run(&mut self, game: &mut Game, input: &mut Input) {
        let config = game.get_config();

        let fb_w = config.x().max(1) as u32;
        let fb_h = config.y().max(1) as u32;

        let event_loop = EventLoop::new().unwrap();

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Asteroids")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(config.x() as u32, config.y() as u32))
                .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
                .build(&event_loop)
                .unwrap()
        );

        set_window_icon_from_png(&window, "data/icons/icon.png");

        
        let win_size = window.inner_size(); // Physical Pixels

        game.get_config_mut().set_actual_size(Vec2::new(win_size.width as usize, win_size.height as usize));

        let surface = SurfaceTexture::new(win_size.width.max(1), win_size.height.max(1), window.clone());

        let mut pixels = Pixels::new(fb_w, fb_h, surface).unwrap();
        pixels.enable_vsync(false);

        let egui_ctx = egui::Context::default();
        let viewport_id = egui_ctx.viewport_id();


        game.get_assets_db_mut().load_dynamic(&egui_ctx);
        game.open_default_scene();
        
        let mut egui_state = EguiWinitState::new(
            egui_ctx,
            viewport_id,
            &window,
            Some(window.scale_factor() as _),
            None,
        );


        let mut egui_renderer = EguiRenderer::new(
            pixels.device(),
            pixels.surface_texture_format(),
            None,
            1,
        );
        event_loop.set_control_flow(ControlFlow::Poll);


        let mut last = Instant::now();
        event_loop.run(move |event, elwt| match event {

            Event::WindowEvent { ref event, .. } => {
                let consumed = egui_state.on_window_event(&window, event).consumed;

                match event {
                    WindowEvent::KeyboardInput { event, .. } => {
                        let is_down = event.state == ElementState::Pressed;

                        if let PhysicalKey::Code(code) = event.physical_key {
                            input.on_key(code, is_down);
                        }
                    }
                    WindowEvent::Resized(size) => {
                        let w = size.width.max(1);
                        let h = size.height.max(1);

                        pixels.resize_surface(w, h).unwrap();
                        pixels.resize_buffer(w, h).unwrap();

                        game.get_screen_mut().resize(w, h);
                    }

                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        egui_state.egui_ctx().set_pixels_per_point(*scale_factor as f32);
                    }
                    WindowEvent::CloseRequested => { elwt.exit(); }
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        let mut dt = (now - last).as_secs_f32();
                        last = now;
                        if dt > 0.1 { dt = 0.1; }


                        let egui_input = egui_state.take_egui_input(&window);
                        egui_state.egui_ctx().begin_frame(egui_input);


                        {
                            ///LOGIC
                            input.update(dt);
                            if (!game.update_game(dt, &egui_state.egui_ctx(), &input)) {
                                elwt.exit();
                            }
                            blit_u32_to_rgba_bytes(pixels.frame_mut(), game.get_screen().get_buffer());
                        }

                        let full = egui_state.egui_ctx().end_frame();

                        let egui::FullOutput {
                            textures_delta,
                            shapes,
                            pixels_per_point,
                            platform_output,
                            ..
                        } = full;

                        egui_state.handle_platform_output(&window, platform_output);


                        let paint_jobs = egui_state
                            .egui_ctx()
                            .tessellate(shapes, pixels_per_point);

                        let win_size = window.inner_size();
                        let screen_descriptor = egui_wgpu::ScreenDescriptor {
                            size_in_pixels: [win_size.width, win_size.height],
                            pixels_per_point: full.pixels_per_point,
                        };

                        for (id, image_delta) in &textures_delta.set {
                            egui_renderer.update_texture(pixels.device(), pixels.queue(), *id, image_delta);
                        }
                        for id in &textures_delta.free {
                            egui_renderer.free_texture(id);
                        }


                        let res = pixels.render_with(|encoder, render_target, context| {
                            // 1) rendering pixels
                            context.scaling_renderer.render(encoder, render_target);

                            // 2) prepare egui buffer
                            let _user_cmds = egui_renderer.update_buffers(
                                pixels.device(),
                                pixels.queue(),
                                encoder,
                                &paint_jobs,
                                &screen_descriptor,
                            );

                            // 3) overlay draw egui
                            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("egui_pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: render_target,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                            egui_renderer.render(&mut pass, &paint_jobs, &screen_descriptor);

                            Ok(())
                        });

                        if res.is_err() {}
                    }

                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }).unwrap();
    }
}

fn blit_u32_to_rgba_bytes(dst_rgba: &mut [u8], src_u32: &[u32]) {
    for (dst, &p) in dst_rgba.chunks_exact_mut(4).zip(src_u32.iter()) {
        let r = ((p >> 16) & 0xFF) as u8;
        let g = ((p >> 8) & 0xFF) as u8;
        let b = (p & 0xFF) as u8;
        dst[0] = r;
        dst[1] = g;
        dst[2] = b;
        dst[3] = 0xFF;
    }
}

fn load_window_icon(path: impl AsRef<Path>) -> Result<Icon, Box<dyn Error>> {
    let img = image::open(path)?.into_rgba8();
    let (w, h) = img.dimensions();
    let rgba = img.into_raw(); // Vec<u8> RGBA

    let icon = Icon::from_rgba(rgba, w, h)?;
    Ok(icon)
}
pub fn set_window_icon_from_png(window: &Window, path: impl AsRef<Path>) {
    match load_window_icon(path) {
        Ok(icon) => window.set_window_icon(Some(icon)),
        Err(e) => eprintln!("icon load failed: {e}"),
    }
}