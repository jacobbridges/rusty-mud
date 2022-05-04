pub mod cogs;
pub mod game;
pub mod utils;


fn main() {
    println!("Creating the game object");
    let mut g = game::Game::new();

    loop {
        g.tick()
    }
}
