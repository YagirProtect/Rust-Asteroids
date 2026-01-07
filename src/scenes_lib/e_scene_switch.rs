use crate::scenes_lib::e_sceneid::SceneId;

pub enum SceneSwitch {
    None,
    Switch(SceneId),
    Quit
}