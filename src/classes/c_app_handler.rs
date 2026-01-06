use crate::classes::c_game::Game;

pub struct AppHandler {

}


impl AppHandler {
    pub fn new() -> AppHandler {
        AppHandler {}
    }


    pub fn run(mut self, mut game: Game) {

        

        // тут весь winit loop
        // но наружу он выдаёт нормальные вызовы:
        // game.update(dt, &input)
        // game.render(&mut screen)
        // game.ui(&egui_ctx)
        // present(screen)
    }
}