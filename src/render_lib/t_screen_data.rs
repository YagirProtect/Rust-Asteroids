use egui::vec2;
use vek::Vec2;

pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            buffer: vec![0; (width * height)]
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.width = w as usize;
        self.height = h as usize;
        self.buffer.resize(self.width * self.height, 0);
    }

    pub fn flush(&mut self) {
        self.buffer.fill(15);
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn center(&self) -> Vec2<f32> { Vec2::new(self.width as f32 / 2.0, self.height as f32 / 2.0) }

    pub fn get_buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}