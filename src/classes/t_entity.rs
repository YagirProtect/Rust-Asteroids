use crate::classes::c_input::Input;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_drawable::Drawable;

pub trait Entity : Drawable{
    
    fn set_entity_id(&mut self, entity_id: u32);
    
    fn get_entity_id(&self) -> u32;
    
    fn update(&mut self, delta_time: f32, input: &Input, config: &Config){
        
    }
}