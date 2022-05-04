use specs::prelude::*;
use specs::world::Index as EntityId;

use crate::game::{Game, RunState};
use crate::game::components;
use crate::game::map::{ExitDirection, Map, Room, RoomId};
use crate::utils;

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

    // Pick something up
    Get(String),

    // Get an item from a container
    GetFrom(String, String),

    // List items in the player's inventory
    Inventory,

    // List items in an inventory
    LookIn(String),

    // Put item in an inventory
    PutIn(String, String),

    // Drop item
    Drop(String),

    // Print some help docs
    Help,

    // Unknown command
    Unknown
}

fn get_enum_for_input_string(input: &str) -> Input {
    use regex::Regex;

    let input_re = Regex::new(
        r#"(?x)
        (next)$ |
        (previous)$ |
        (prev)$ |
        (look)$ |
        (look)\s+(in)\s+(\w+)$ |
        (look)(?:\s+at)?\s+(\w+)$ |
        (get)\s+(\w+)(?:\s+from)?\s+(\w+)$ |
        (get)\s+(\w+)$ |
        (inv)$ |
        (inventory)$ |
        (put)\s+(\w+)(?:\s+in)?\s+(\w+)$ |
        (drop)\s+(\w+)$ |
        (help)$
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
        Some(["look"]) => Input::Look,
        Some(["look", "in", x]) => Input::LookIn(x.to_string()),
        Some(["look", x]) => Input::LookAt(x.to_string()),
        Some(["get", x]) => Input::Get(x.to_string()),
        Some(["get", x, y]) => Input::GetFrom(x.to_string(), y.to_string()),
        Some(["inv"]) | Some(["inventory"]) => Input::Inventory,
        Some(["put", x, y]) => Input::PutIn(x.to_string(), y.to_string()),
        Some(["drop", x]) => Input::Drop(x.to_string()),
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
            let mut apply_moves = game.world.write_storage::<components::ApplyMove>();
            let mut inrooms = game.world.write_storage::<components::InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let room: &Room = map.room(&inroom.room);
            match room.exit(ExitDirection::Next) {
                Some(g) => {
                    if g.is_locked() {
                        println!("That path is locked!");
                    } else {
                        apply_moves
                            .insert(player, components::ApplyMove{room: g.to()})
                            .expect("Unable to insert");
                    }
                }
                None => println!("Already at the last room!"),
            }
        }
        Input::Previous => {
            let mut apply_moves = game.world.write_storage::<components::ApplyMove>();
            let mut inrooms = game.world.write_storage::<components::InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let room: &Room = map.room(&inroom.room);
            match room.exit(ExitDirection::Previous) {
                Some(g) => {
                    if g.is_locked() {
                        println!("That path is locked!");
                    } else {
                        apply_moves
                            .insert(player, components::ApplyMove{room: g.to()})
                            .expect("Unable to insert");
                    }
                }
                None => println!("Already at the first room!"),
            }
        }
        Input::Look => {
            let mut inrooms = game.world.write_storage::<components::InRoom>();
            let inroom = inrooms.get_mut(player).unwrap();
            let map = game.world.read_resource::<Map>();
            let descriptions = game.world.read_storage::<components::Description>();
            let room: &Room = map.room(&inroom.room);
            println!("{}", room.description(
                &game.world.entities(),
                &mut inrooms,
                &descriptions,
            ));

            return RunState::AwaitingInput
        }
        Input::LookAt(x) => {
            let mut lookables: Vec<Entity> = Vec::new();
            lookables.extend(get_entities_in_room(
                get_player_room_id(&player, &game.world),
                &game.world,
            ));
            lookables.extend(get_entities_from_entity_inventory(&player, &game.world));

            let mut bitset = BitSet::new();
            for entity in lookables {
                bitset.add(entity.id());
            }

            let mut target: Option<String> = None;
            for (_, desc) in (&bitset, &game.world.read_storage::<components::Description>()).join() {
                if desc.glance.starts_with(x.as_str()) {
                    target = Some(desc.description.clone());
                    break;
                }
            }

            if let Some(desc) = target {
                println!("{}", desc);
            } else {
                println!("Could not find \"{}\"", x);
            }
            return RunState::AwaitingInput
        }
        Input::LookIn(x) => {
            let storages = game.world.read_storage::<components::Storage>();
            let ds = game.world.read_storage::<components::Description>();
            let room_id = get_player_room_id(&player, &game.world);
            let entities_in_room = get_entities_in_room(room_id, &game.world);
            let mut target: Option<Entity> = None;
            for entity in entities_in_room {
                let desc_opt = ds.get(entity);
                let store_opt = storages.get(entity);
                match (desc_opt, store_opt) {
                    (Some(desc), Some(_store)) => {
                        if desc.glance.starts_with(x.as_str()) {
                            target = Some(entity);
                            break;
                        }
                    },
                    (Some(desc), None) => {
                        if desc.glance.starts_with(x.as_str()) {
                            println!("You can't store items in that!");
                            return RunState::PlayerTurn
                        }
                    }
                    (None, _) => {}
                }
            }

            if let Some(e) = target {
                let container = storages.get(e).unwrap();
                let container_desc = ds.get(e).unwrap();
                if container.items.len() > 0 {
                    println!("{} inventory -----", &container_desc.glance);
                    for item in &container.items {
                        let item_desc = ds.get(*item)
                            .expect(format!("Expected item {} in container {} to have a Description component!", item.id(), e.id()).as_str());
                        println!("- {}", &item_desc.glance);
                    }
                } else {
                    println!("The {} is empty", &container_desc.glance);
                }
            } else {
                println!("Nothing in the room like \"{}\"", x);
            }
        }
        Input::Get(x) => {
            let entities = game.world.entities();
            let items = game.world.read_storage::<components::Item>();
            let inrooms = game.world.write_storage::<components::InRoom>();
            let player_inroom = inrooms.get(player).unwrap();
            let descriptions = game.world.read_storage::<components::Description>();
            let mut target: Option<Entity> = None;
            for (entity, _item, inroom, description) in (&entities, &items, &inrooms, &descriptions).join() {
                if inroom.room == player_inroom.room {
                    if description.glance.starts_with(x.as_str()) {
                        target = Some(entity);
                        break;
                    }
                }
            }
            if let Some(e) = target {
                let mut changes = game.world.write_storage::<components::ApplyInventoryChange>();
                changes
                    .insert(e, components::ApplyInventoryChange { from_container: None, to_container: Some(player) })
                    .expect("Unable to insert");
            } else {
                println!("Not able to pickup \"{}\"", x);
                return RunState::AwaitingInput
            }
        }
        Input::GetFrom(item_name, container_name) => {
            let ds = game.world.read_storage::<components::Description>();

            let room_id = get_player_room_id(&player, &game.world);
            let room_items = get_entities_in_room(room_id, &game.world);
            let mut target_container_opt: Option<(Entity, String)> = None;
            {
                let storages = game.world.read_storage::<components::Storage>();
                for room_item in room_items {
                    if let Some(desc) = ds.get(room_item) {
                        if desc.glance.starts_with(container_name.as_str()) {
                            if storages.get(room_item).is_some() {
                                target_container_opt = Some((room_item, desc.glance.clone()));
                                break;
                            } else {
                                println!("{} cannot store items", desc.glance.clone());
                                return RunState::AwaitingInput
                            }
                        }
                    }
                }
            }

            if let None = target_container_opt {
                println!("Could not find \"{}\" in the current room", container_name.as_str());
                return RunState::AwaitingInput
            }

            let (container, container_glance) = target_container_opt.unwrap();
            let container_items = get_entities_from_entity_inventory(&container, &game.world);
            let mut target_item_opt: Option<Entity> = None;
            for container_item in container_items {
                let desc = ds.get(container_item)
                    .expect("Expected item in container inventory to have Description component");
                if desc.glance.starts_with(item_name.as_str()) {
                    target_item_opt = Some(container_item);
                    break;
                }
            }

            if let None = target_item_opt {
                println!("Nothing in the {} like \"{}\"", container_glance, item_name);
                return RunState::AwaitingInput
            }

            let item = target_item_opt.unwrap();
            let mut changes = game.world.write_storage::<components::ApplyInventoryChange>();
            changes.insert(item, components::ApplyInventoryChange {
                from_container: Some(container),
                to_container: Some(player),
            }).expect("Could not insert ApplyInventoryChange");
        }
        Input::Inventory => {
            let storages = game.world.read_storage::<components::Storage>();
            let ds = game.world.read_storage::<components::Description>();
            let player_storage = storages.get(player).expect("Expected player to have a storage component");
            let mut items : Vec<String> = Vec::new();
            for entity in &player_storage.items {
                let desc = ds.get(*entity)
                    .expect(format!("Entity {} in player's inventory does not have the Description component!", entity.id()).as_str());
                items.push(desc.glance.clone());
            }
            if items.len() > 0 {
                println!("Your inventory -----");
                for item in items {
                    println!("- {}", item);
                }
            } else {
                println!("Your inventory is empty!")
            }

            return RunState::AwaitingInput
        }
        Input::PutIn(item_name, container_name) => {
            let player_items = get_entities_from_entity_inventory(&player, &game.world);
            let ds = game.world.read_storage::<components::Description>();

            let mut target_item_opt: Option<Entity> = None;
            for player_item in player_items {
                let desc = ds.get(player_item)
                    .expect("Expected item in player inventory to have Description component");
                if desc.glance.starts_with(item_name.as_str()) {
                    target_item_opt = Some(player_item);
                    break;
                }
            }

            if let None = target_item_opt {
                println!("Nothing in your inventory like \"{}\"", item_name);
                return RunState::AwaitingInput
            }

            let room_id = get_player_room_id(&player, &game.world);
            let room_items = get_entities_in_room(room_id, &game.world);
            let mut target_container_opt: Option<Entity> = None;
            let storages = game.world.read_storage::<components::Storage>();
            for room_item in room_items {
                if let Some(desc) = ds.get(room_item) {
                    if desc.glance.starts_with(container_name.as_str()) {
                        if storages.get(room_item).is_some() {
                            target_container_opt = Some(room_item);
                            break;
                        } else {
                            println!("{} cannot store items", desc.glance.clone());
                            return RunState::AwaitingInput
                        }
                    }
                }
            }

            if let None = target_container_opt {
                println!("Could not find \"{}\" in the current room", container_name.as_str());
                return RunState::AwaitingInput
            }

            let item = target_item_opt.unwrap();
            let container = target_container_opt.unwrap();
            let mut changes = game.world.write_storage::<components::ApplyInventoryChange>();
            changes.insert(item, components::ApplyInventoryChange {
                from_container: Some(player),
                to_container: Some(container),
            }).expect("Could not insert ApplyInventoryChange");
        }
        Input::Drop(x) => {
            let inv_items = get_entities_from_entity_inventory(&player, &game.world);
            let ds = game.world.read_storage::<components::Description>();
            let mut target_opt: Option<Entity> = None;
            for item in inv_items {
                if let Some(desc) = ds.get(item) {
                    if desc.glance.starts_with(x.as_str()) {
                        target_opt = Some(item);
                        break;
                    }
                }
            }

            if let Some(target) = target_opt {
                let mut changes = game.world.write_storage::<components::ApplyInventoryChange>();
                changes.insert(target, components::ApplyInventoryChange {
                    from_container: Some(player),
                    to_container: None,
                }).expect("Could not insert ApplyInventorChange");
            } else {
                println!("Nothing in your inventory like \"{}\"", x);
                return RunState::AwaitingInput
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


fn get_player_room_id(player: &Entity, ecs: &World) -> RoomId {
    let inrooms = ecs.read_storage::<components::InRoom>();
    let inroom = inrooms.get(*player)
        .expect("Expected player to be in a room?!");

    inroom.room.clone()
}


fn get_entities_in_room(room_id: RoomId, ecs: &World) -> Vec<Entity> {
    let inrooms = ecs.read_storage::<components::InRoom>();
    let mut entities_in_room: Vec<Entity> = Vec::new();

    for (entity, inroom) in (&ecs.entities(), &inrooms).join() {
        if inroom.room == room_id {
            entities_in_room.push(entity)
        }
    }

    return entities_in_room;
}

fn get_entities_from_entity_inventory(player: &Entity, ecs: &World) -> Vec<Entity> {
    let storages = ecs.read_storage::<components::Storage>();
    let entity_store = storages.get(*player)
        .expect("Expected Storage component on player!");

    return entity_store.items.clone()
}
