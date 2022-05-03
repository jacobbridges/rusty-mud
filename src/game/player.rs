use specs::prelude::*;
use specs::world::Index as EntityId;

use crate::game::{Game, RunState};
use crate::game::components::{ApplyMove, Description, InRoom};
use crate::game::map::{ExitDirection, Map, Room};

#[derive(Clone, Debug)]
pub struct PlayerInput {
    pub player_id: EntityId,
    pub input: String,
}

#[derive(PartialEq, Clone)]
enum Input {
    // Advance to the next room
    Next,
    // Backtrack to previous room
    Previous,
    // Look around the room
    Look,
    // Look at (something)
    LookAt(String),
    // Print some help docs
    Help,
    // Unknown command
    Unknown
}

fn get_enum_for_input_string(input: &str) -> Input {
    use regex::Regex;
    println!("get_enum_for_input_string({:?})", input);

    let input_re = Regex::new(
        r#"(?x)
        (next)\s?$ |
        (previous)\s?$ |
        (prev)\s?$ |
        (l)\s?$ |
        (look)(?:\s+at)?$ |
        (look)(?:\s+at)?\s+(\w+)$ |
        (help)\s?$
        "#
    ).unwrap();

    let captures = input_re.captures(input).map(|captures| {
        captures
            .iter() // All the captured groups
            .skip(1) // Skipping the complete match
            .flat_map(|c| c) // Ignoring all empty optional matches
            .map(|c| c.as_str()) // Grab the original strings
            .collect::<Vec<_>>() // Create a vector
    });

    match captures.as_ref().map(|c| c.as_slice()) {
        Some(["next"]) => Input::Next,
        Some(["previous"]) | Some(["prev"]) => Input::Previous,
        Some(["look"]) | Some(["l"]) => Input::Look,
        Some(["look", x]) => Input::LookAt(x.to_string()),
        Some(["help"]) => Input::Help,
        x => {
            println!("Unknown input: {:?}", x);
            Input::Unknown
        },
    }
}

pub fn handle_player_input(game: &mut Game) -> RunState {
    use text_io::read;

    let player = game.world.entities().entity(game.player_id);

    println!("Please input a command");
    let input: String = read!("{}\n");
    match get_enum_for_input_string(&input.trim()) {
        Input::Next => {
            let mut apply_moves = game.world.write_storage::<ApplyMove>();
            let mut inrooms = game.world.write_storage::<InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let room: &Room = map.room(&inroom.room);
            match room.exit(ExitDirection::Next) {
                Some(g) => {
                    if g.is_locked() {
                        println!("That path is locked!");
                    } else {
                        apply_moves
                            .insert(player, ApplyMove{room: g.to()})
                            .expect("Unable to insert");
                    }
                }
                None => println!("Already at the last room!"),
            }
        }
        Input::Previous => {
            let mut apply_moves = game.world.write_storage::<ApplyMove>();
            let mut inrooms = game.world.write_storage::<InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let room: &Room = map.room(&inroom.room);
            match room.exit(ExitDirection::Previous) {
                Some(g) => {
                    if g.is_locked() {
                        println!("That path is locked!");
                    } else {
                        apply_moves
                            .insert(player, ApplyMove{room: g.to()})
                            .expect("Unable to insert");
                    }
                }
                None => println!("Already at the first room!"),
            }
        }
        Input::Look => {
            let mut inrooms = game.world.write_storage::<InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let descriptions = game.world.read_storage::<Description>();
            let room: &Room = map.room(&inroom.room);
            println!("{}", room.description(
                &game.world.entities(),
                &mut inrooms,
                &descriptions,
            ));
        }
        Input::LookAt(x) => {
            let mut inrooms = game.world.write_storage::<InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let descriptions = game.world.read_storage::<Description>();
            let map = game.world.read_resource::<Map>();
            let room: &Room = map.room(&inroom.room);
            let mut target: Option<&Description> = None;
            for (inroom, description) in (&mut inrooms, &descriptions).join() {
                if inroom.room == room.id() {
                    if description.glance.starts_with(x.as_str()) {
                        target = Some(description)
                    }
                }
            }
            match target {
                Some(desc) => println!("{}", desc.description),
                None => {
                    println!("Could not find \"{}\"", x);
                    return RunState::AwaitingInput
                }
            }
        }
        Input::Help => {
            println!("{}", [
                "Available actions:".to_string(),
                format!("{: <14}{}", "next", "Go to the next room"),
                format!("{: <14}{}", "prev", "Go to the previous room"),
                format!("{: <14}{}", "look", "Describe the current room"),
                format!("{: <14}{}", "look at ____", "Describe an object by name"),
            ].join("\n"));
            return RunState::AwaitingInput
        }
        _ => {
            println!("No such command!");
            return RunState::AwaitingInput
        }
    }

    RunState::PlayerTurn
}
