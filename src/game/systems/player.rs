use specs::{Entities, ReadStorage, WriteStorage, Write, System, Entity, WorldExt, Join, Read};
use crate::game::components::{InRoom, Description, Player};
use crate::game::map::{Map, Room, Gate, ExitDirection};
use crate::game::PlayerInput;


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

pub struct PlayerSystem;

impl<'a> PlayerSystem {
    fn handle_player_input(&mut self, player_input: PlayerInput, map: &mut Map, entity_storage: &Entities<'a>, in_room_storage: &mut WriteStorage<'a, InRoom>, description_storage: &ReadStorage<'a, Description>) {
        let input = get_enum_for_input_string(&player_input.input.as_str());
        let player_ent = entity_storage.entity(player_input.player_id.clone());

        match input {
            Input::Next => {
                let mut in_room_comp = in_room_storage.get_mut(player_ent).unwrap();
                let room: &Room = map.room(&in_room_comp.room);
                match room.exit(ExitDirection::Next) {
                    Some(g) => {
                        if g.is_locked() {
                            println!("That path is locked!");
                        } else {
                            println!("Advancing player {} to room {}", player_ent.id().clone(), room.id());
                            in_room_comp.room = g.to();
                        }
                    },
                    None => println!("Already at the last room!"),
                }
                // in_room_comp.room = match current_room_comp.get_next_room_id() {
                //     Some(room_id) => {
                //         println!("Advancing player {} to room {}", player_ent.id().clone(), room_id);
                //         room_id
                //     },
                //     None => {
                //         // TODO: Display some message like "already at the last room!"
                //         println!("Already at the last room!");
                //         current_room_ent.id()
                //     }
                // }
            },
            Input::Previous => {
                let mut in_room_comp = in_room_storage.get_mut(player_ent).unwrap();
                let room: &Room = map.room(&in_room_comp.room);
                match room.exit(ExitDirection::Previous) {
                    Some(g) => {
                        if g.is_locked() {
                            println!("That path is locked!");
                        } else {
                            println!("Retreating player {} to room {}", player_ent.id().clone(), room.id());
                            in_room_comp.room = g.to();
                        }
                    },
                    None => println!("Already at the first room!"),
                }
            },
            Input::Look => {
                let mut in_room_comp = in_room_storage.get_mut(player_ent).unwrap();
                let room: &mut Room = map.room(&in_room_comp.room);
                println!("{}", room.description(
                    entity_storage,
                    in_room_storage,
                    description_storage,
                ));
            },
            Input::LookAt(x) => {
                let in_room_comp = in_room_storage.get(player_ent).unwrap();
                let mut target: Option<&Description> = None;
                for (inroom, description) in (&*in_room_storage, description_storage).join() {
                    if inroom.room == in_room_comp.room {
                        if description.glance.starts_with(x.as_str()) {
                            target = Some(description)
                        }
                    }
                }
                match target {
                    Some(desc) => println!("{}", desc.description),
                    None => println!("Could not find \"{}\"", x),
                }
            },
            Input::Help => {
                println!("{}", [
                    "Available actions:".to_string(),
                    format!("{: <14}{}", "next", "Go to the next room"),
                    format!("{: <14}{}", "prev", "Go to the previous room"),
                    format!("{: <14}{}", "look", "Describe the current room"),
                    format!("{: <14}{}", "look at ____", "Describe an object by name"),
                ].join("\n"))
            }
            _ => {
                // TODO: Display some message like "no such command!"
                println!("No such command!");
            }
        };
    }
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Entities<'a>,
        Option<Write<'a, Vec<PlayerInput>>>,
        Option<Write<'a, Map>>,
        WriteStorage<'a, InRoom>,
        ReadStorage<'a, Description>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (entity_storage, player_input_storage, map_storage, mut in_room_storage, description_storage, _p): Self::SystemData) {
        let mut map = map_storage.unwrap();
        match player_input_storage {
            Some(mut list) => {
                println!("*player_input_storage = {:?}", &*list);
                match (*list).pop() {
                    Some(player_input) => {
                        self.handle_player_input(
                            player_input,
                            &mut map,
                            &entity_storage,
                            &mut in_room_storage,
                            &description_storage,
                        );
                    },
                    None => {
                        println!("No player input found in list!");
                    }
                }
            },
            None => {
                println!("No player input found at all!");
            }
        }
    }
}