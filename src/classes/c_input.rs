use std::collections::hash_map::Entry;
use std::collections::HashMap;
use vek::Lerp;
use winit::keyboard::KeyCode;


#[derive(Clone, Debug)]
pub struct InputAxis{
    pub raw: i8,
    pub lerp: f32,

    pub weight: f32
}

impl InputAxis{
    pub fn new(weight: f32) -> InputAxis{
        InputAxis{ raw: 0, lerp: 0.0, weight }
    }
    pub fn add(&mut self, v: i8) {
        self.raw += v;
    }
    pub fn reset(&mut self){
        self.raw = 0;
    }

    pub fn update(&mut self, dt: f32) {
        self.lerp = f32::lerp(self.lerp, self.raw as f32, dt * self.weight);
    }
}


#[derive(Debug, Clone)]
pub struct Input {
    keys_states: HashMap<KeyCode, bool>,
    horizontal: InputAxis,
    vertical: InputAxis,
    fire: InputAxis
}

impl Default for Input {
    fn default() -> Self {
        Self{
            keys_states: HashMap::new(),


            horizontal: InputAxis::new(
                10.0
            ),
            vertical: InputAxis::new(
                5.0
            ),
            fire: InputAxis::new(
                100.0
            )
        }
    }
}

impl Input {

    pub fn on_key(&mut self, key: KeyCode, pressed: bool) {

        match self.keys_states.entry(key) {
            Entry::Occupied(mut e) => {
                *e.get_mut() = pressed;
            },
            Entry::Vacant(e) => {
                e.insert(pressed);
            }
        }
    }

    pub fn get_axis_hor(&self) -> f32{
        self.horizontal.lerp
    }


    pub fn get_axis_ver(&self) -> f32{
        self.vertical.lerp
    }

    pub fn get_fire(&self) -> bool {
        return self.fire.raw >= 1;
    }

    pub fn update(&mut self, dt: f32) {

        self.horizontal.reset();
        self.vertical.reset();
        self.fire.reset();


        if (self.is_key_down(KeyCode::KeyA) || self.is_key_down(KeyCode::ArrowLeft)) {
            self.horizontal.add(-1);
        }
        if (self.is_key_down(KeyCode::KeyD) || self.is_key_down(KeyCode::ArrowRight)) {
            self.horizontal.add(1);
        }
        if (self.is_key_down(KeyCode::KeyW) || self.is_key_down(KeyCode::ArrowUp)) {
            self.vertical.add(1);
        }
        if (self.is_key_down(KeyCode::Space)) {
            self.fire.add(1);
        }

        self.horizontal.update(dt);
        self.vertical.update(dt);
        self.fire.update(dt);
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        if let Some(v) = self.keys_states.get(&key) {
            return v.clone();
        }
        false
    }
}
