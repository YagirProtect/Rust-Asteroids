use crate::classes::c_input::Input;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::uniq_id_lib::c_entity_id::get_uniq_id;

pub trait Scene {
    fn create_scene(&mut self, config: &Config, screen: &Screen);

    fn get_scene_name(&self) -> String;
    fn update(&mut self, dt: f32, input: &Input, config: &Config) -> SceneSwitch {
        self.update_entity(dt, input, config);
        SceneSwitch::None
    }

    fn update_entity(&mut self, dt: f32, input: &Input, config: &Config) {
        for e in self.get_entities_mut().iter_mut() {
            e.update(dt, input, config);
        }
    }


    fn render(&mut self, screen: &mut Screen) {
        for e in self.get_entities_mut().iter_mut() {
            e.draw(screen);
        }
    }
    fn ui(&mut self, _ctx: &egui::Context) {}



    fn get_entities(&self) -> &Vec<Box<dyn Entity>>;
    fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>>;
    fn add_entity(&mut self, mut entity: Box<dyn Entity>){

        entity.set_entity_id(get_uniq_id());
        self.get_entities_mut().push(entity);
    }
    fn remove_entity(&mut self, entity_id: u32){
        for (i, val) in self.get_entities().iter().enumerate() {
            if (val.get_entity_id() == entity_id){
                self.get_entities_mut().remove(i);
                return;
            }
        }
    }
}