use vek::Vec2;

pub struct Transform {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    scale: Vec2<f32>,
    rotation: f32,
}

impl Transform {
    pub fn rotate_to(&mut self, z: f32) {
        self.rotation = z;
    }
}

impl Transform {
    pub fn new(position: Vec2<f32>, scale: Vec2<f32>, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
            velocity: Vec2::new(0.0, 0.0),
        }
    }
    
    
    pub fn set_velocity(&mut self, velocity: Vec2<f32>) {
        self.velocity = velocity;
    }
    
    pub fn get_velocity(&self) -> &Vec2<f32> {
        &self.velocity
    }
    

    pub fn transform_point_to_world(&self, local: Vec2<f32>) -> Vec2<f32> {
        // 1) scale
        let p = Vec2::new(local.x * self.scale.x, local.y * self.scale.y);

        // 2) rotate
        let (s, c) = self.rotation.sin_cos();
        let p = Vec2::new(p.x * c - p.y * s, p.x * s + p.y * c);

        // 3) translate
        p + self.position
    }
    pub fn transform_dir_to_world(&self, local_dir: Vec2<f32>) -> Vec2<f32> {
        let v = Vec2::new(local_dir.x * self.scale.x, local_dir.y * self.scale.y);
        let (s, c) = self.rotation.sin_cos();
        Vec2::new(v.x * c - v.y * s, v.x * s + v.y * c)
    }

    pub fn inverse_transform_point(&self, world: Vec2<f32>) -> Vec2<f32> {
        // world -> local (обратно): translate^-1, rotate^-1, scale^-1
        let p = world - self.position;

        let (s, c) = (-self.rotation).sin_cos();
        let p = Vec2::new(p.x * c - p.y * s, p.x * s + p.y * c);

        Vec2::new(p.x / self.scale.x, p.y / self.scale.y)
    }
}
