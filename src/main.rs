mod mesh_lib;
mod assetsdb_lib;
mod config_lib;
mod transform_lib;
mod classes;
mod render_lib;
mod scenes_lib;
mod uniq_id_lib;
mod collisions_lib;

use crate::classes::c_app_handler::AppHandler;
use crate::classes::c_game::Game;
use crate::classes::c_input::Input;

fn main() {
    let mut input = Input::default();
    let mut game = Game::new();
    let mut apphost = AppHandler::default();
    apphost.run(&mut game, &mut input);
}
