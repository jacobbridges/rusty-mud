// This file is temporary!
//
// The vision is to have hundreds of templates for rooms, mobs, and items which can be combined
// into a re-playable game.
//
// But for now, just to get this project moving, I need a large map with lots of entities to play
// with and program for.
// ----------------------------
use std::collections::BTreeMap;
use specs::{Builder, World, WorldExt};
use crate::game::components;
use crate::game::map;


pub fn generate_map(ecs: &mut World) {
    let mut map = map::Map::new();

    // Create rooms ================================================================================
    let mut room0 = map.create_room(
        "A train car, transformed into a luxurious atrium either by experienced interior \
        designers or story-wizards. The lush carpet pads the footfalls of everyone in the room."
    );
    let mut room1 = map.create_room(
        "A martini lounge."
    );
    let mut room2 = map.create_room(
        "Some kind of jazz club. Low lighting, light jazz music without an origin, and \
        a small stage to the side for performances. There seems to be a lack of audience tho.."
    );
    let mut room3 = map.create_room(
        "A mostly empty storage railcar. A lanky figure with pale blue skin and dark \
        orange hair is anxiously pacing the room."
    );

    // Create gates ================================================================================
    room0.add_exit(
        map::ExitDirection::Next,
        room1.as_gate("Hallway leading to the next train car")
    );
    room1.add_exit(
        map::ExitDirection::Previous,
        room0.as_gate("Hallway leading to the previous train car")
    );
    room1.add_exit(
        map::ExitDirection::Next,
        room2.as_gate("Curved doorway to the next train car")
    );
    room2.add_exit(
        map::ExitDirection::Previous,
        room1.as_gate("Curved doorway to the previous train car")
    );
    room2.add_exit(
        map::ExitDirection::Next,
        room3.as_gate_locked(
            "A locked door with a card-reader slot",
            vec![],
        ),
    );

    // Create entities =============================================================================
    let receptionist = ecs.create_entity()
        .with(components::InRoom {room: room0.id()})
        .with(components::Npc {})
        .with(components::Description {
            name: Some("Clarice Nimpton".to_string()),
            glance: "receptionist".to_string(),
            description: "An ogrodon female welcoming all new-comers".to_string(),
        })
        .build();
    let passed_out_old_man = ecs.create_entity()
        .with(components::InRoom {room: room0.id()})
        .with(components::Npc {})
        .with(components::Description {
            name: Some("Mflel Bgargar".to_string()),
            glance: "old man".to_string(),
            description: "An elderly man passed out in a chair".to_string(),
        })
        .build();
    let room3_key = ecs.create_entity()
        .with(components::Item {})
        .with(components::Description {
            name: None,
            glance: "blue keycard".to_string(),
            description: "A semi-translucent blue keycard with engraved letters \"\
            C-4\" in the corner".to_string()
        })
        .build();
    room2.exits.get_mut(&map::ExitDirection::Next).unwrap().add_key(room3_key);
    let room0_bin = ecs.create_entity()
        .with(components::InRoom { room: room0.id() })
        .with(components::Description {
            name: None,
            glance: "bin".to_string(),
            description: "A plastic bin along the wall with a label: \"Lost & Found\"".to_string(),
        })
        .with(components::Storage { items: vec![room3_key] })
        .build();

    for robot_name in vec!["1-L3GG3D-J03", "2-F1NG3RD-B0B", "B1RD-3Y3-B1LLY"] {
        ecs.create_entity()
            .with(components::Npc {})
            .with(components::InRoom { room: room1.id() })
            .with(components::Description {
                name: Some(robot_name.to_string()),
                glance: "robot pirate".to_string(),
                description: "A decommissioned kitchen droid equipped with a buccaneer's hat".to_string(),
            })
            .build();
    }



    let mut first_room = map.create_room("This is the first room");
    let mut second_room = map.create_room("This is the second room");
    second_room
        .add_exit(map::ExitDirection::Previous, first_room.as_gate("Hallway leading to the previous train car."));
    first_room
        .add_exit(map::ExitDirection::Next, second_room.as_gate("Hallway leading to the next train car."));

    map.set_spawn(first_room.id());
    map.rooms.insert(first_room.id(), first_room);
    map.rooms.insert(second_room.id(), second_room);
}