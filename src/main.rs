
#[macro_use]
extern crate glium;

#[path = "./game/dummy_game.rs"]
pub mod my_game;

fn main(){
    let mut dummy_game = my_game::DummyGame::new("myGame");
    dummy_game.start();
}