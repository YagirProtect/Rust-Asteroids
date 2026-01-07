use crate::classes::c_game::Game;

use std::{error::Error, time::Instant};
use std::sync::Arc;
use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State as EguiWinitState;
use pixels::{wgpu, Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::event::ElementState;
use winit::keyboard::PhysicalKey;
use winit::window::WindowButtons;
use crate::classes::c_input::Input;

#[derive(Copy, Clone, Default)]
pub struct AppHandler {

}



impl AppHandler {
    pub fn run(&mut self, game: &mut Game, input: &mut Input) {
        let config = game.get_config();

        let event_loop = EventLoop::new().unwrap();

        let window = Arc::new(
            WindowBuilder::new()
            .with_title("Asteroids")
            .with_resizable(false)
            .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
            .build(&event_loop)
            .unwrap()
        );
        let surface = SurfaceTexture::new(config.x().max(1) as u32, config.y().max(1) as u32, window.clone());

        // Теперь это Pixels<'static>
        let mut pixels: Pixels<'static> = Pixels::new(config.x().max(1) as u32, config.y().max(1) as u32, surface).unwrap();
        pixels.enable_vsync(false);


        let egui_ctx = egui::Context::default();
        let viewport_id = egui_ctx.viewport_id();

        let mut egui_state = EguiWinitState::new(
            egui_ctx,
            viewport_id,
            &window,
            Some(window.scale_factor() as _),
            None,
        );



        // egui-wgpu renderer: важно использовать формат surface, который реально рисуется в окно.
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

                    WindowEvent::CloseRequested => { elwt.exit(); }
                    WindowEvent::RedrawRequested => {
                        let now = Instant::now();
                        let mut dt = (now - last).as_secs_f32();
                        last = now;
                        if dt > 0.1 { dt = 0.1; }


                        let egui_input = egui_state.take_egui_input(&window);
                        egui_state.egui_ctx().begin_frame(egui_input);


                        {///LOGIC
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

                        // обновляем текстуры egui (шрифты/картинки)
                        for (id, image_delta) in &textures_delta.set {
                            egui_renderer.update_texture(pixels.device(), pixels.queue(), *id, image_delta);
                        }
                        for id in &textures_delta.free {
                            egui_renderer.free_texture(id);
                        }



                        let res = pixels.render_with(|encoder, render_target, context| {
                            // 1) отрисовать пиксельный буфер (то, что ты залил в pixels.frame_mut())
                            context.scaling_renderer.render(encoder, render_target);

                            // 2) подготовить буферы egui
                            let _user_cmds = egui_renderer.update_buffers(
                                pixels.device(),
                                pixels.queue(),
                                encoder,
                                &paint_jobs,
                                &screen_descriptor,
                            );

                            // 3) отрисовать egui поверх (LoadOp::Load — НЕ затирать картинку)
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

                        if res.is_err() {
                            // например, если сворачиваешь окно/теряется surface
                            // можно просто игнорировать кадр
                        }

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
    // пример: считаем, что src = 0x00RRGGBB (без альфы)
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