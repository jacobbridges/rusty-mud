pub mod cogs;
pub mod game;


fn main() {
    use text_io::read;

    println!("Creating the game object");
    let mut g = game::Game::new();
    println!("Creating the player");
    let player_id = g.create_player();

    loop {
        // Stuff to do during game loop
        {
            println!("Please input a command");
            let input: String = read!("{}\n");
            g.player_input(player_id.clone(), input.trim());
            g.tick()
        }
    }
}
