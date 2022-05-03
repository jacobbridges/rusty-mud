pub mod cogs;
pub mod game;


fn main() {
    println!("Creating the game object");
    let mut g = game::Game::new();

    loop {
        g.tick()
    }
}
