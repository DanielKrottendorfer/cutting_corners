extern crate image;
extern crate cgmath;
extern crate collada;

#[macro_use] extern crate glium;

extern  crate assimp;

#[path = "./game/engine/cc_game_engine.rs"]
pub mod my_game_engine;

fn main(){
    let ga = my_game_engine::my_game_logic::CCGame::new();
    let mut ge = my_game_engine::CCGameEngine::new(ga,"test123",false);
    ge.start();
}
