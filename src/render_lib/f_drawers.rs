use std::rc::Rc;
use egui::load::SizedTexture;
use vek::Vec2;
use crate::render_lib::t_screen_data::Screen;
use crate::sprite_lib::c_sprite::SpriteTex;

pub fn draw_line_thick(screen: &mut Screen, x0f: f32, y0f: f32, x1f: f32, y1f: f32, thickness: i32, color: u32) {
    let x0 = x0f.round() as i32;
    let y0 = y0f.round() as i32;
    let x1 = x1f.round() as i32;
    let y1 = y1f.round() as i32;

    let t = thickness.max(1);
    // оффсеты дают ровно t пикселей: например t=4 => -1..=2 (4 штуки)
    let neg = (t - 1) / 2;
    let pos = t / 2;

    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(screen, x1, y1, x0, y0, neg, pos, color);
        } else {
            plot_line_low(screen, x0, y0, x1, y1, neg, pos, color);
        }
    } else {
        if y0 > y1 {
            plot_line_high(screen, x1, y1, x0, y0, neg, pos, color);
        } else {
            plot_line_high(screen, x0, y0, x1, y1, neg, pos, color);
        }
    }

    fn plot_line_low(screen: &mut Screen, x0: i32, y0: i32, x1: i32, y1: i32, neg: i32, pos: i32, color: u32) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }

        let mut d = (2 * dy) - dx;
        let mut y = y0;

        for x in x0..=x1 {
            // толщина: вертикальный штамп
            for oy in -neg..=pos {
                set_pixel(screen, x, y + oy, color);
            }

            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }

    fn plot_line_high(screen: &mut Screen, x0: i32, y0: i32, x1: i32, y1: i32, neg: i32, pos: i32, color: u32) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut d = (2 * dx) - dy;
        let mut x = x0;

        for y in y0..=y1 {
            // толщина: горизонтальный штамп
            for ox in -neg..=pos {
                set_pixel(screen, x + ox, y, color);
            }

            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }
}

#[inline]
fn wrap_i32(v: i32, m: i32) -> usize {
    if (0..m).contains(&v) {
        return v as usize;
    }
    if (-m..0).contains(&v) {
        return (v + m) as usize;
    }
    if (m..2 * m).contains(&v) {
        return (v - m) as usize;
    }
    v.rem_euclid(m) as usize
}

pub fn set_pixel(screen: &mut Screen, x: i32, y: i32, color: u32) {
    let w = screen.width() as i32;
    let h = screen.height() as i32;

    let x = wrap_i32(x, w);
    let y = wrap_i32(y, h);

    let pos = x + screen.width() * y;
    screen.get_buffer_mut()[pos] = color;
}

pub fn ui_draw_icon(ui: &mut egui::Ui, sprite: &Rc<SpriteTex>, size: Vec2<f32>) {
    let tex = sprite.get_gui_texture();
    let st = SizedTexture::new(tex.id(), tex.size_vec2());
    ui.add(egui::Image::from_texture(st).fit_to_exact_size(egui::vec2(size.x, size.y)));
}
