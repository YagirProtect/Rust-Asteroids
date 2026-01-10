use crate::scenes_lib::e_sceneid::SceneId;

#[derive(Default, Copy, Clone)]
pub enum SceneSwitch {
    #[default]
    None,
    Switch(SceneId),
    Quit
}