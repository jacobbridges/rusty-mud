mod components;
mod systems;
mod map;

use std::collections::BTreeMap;
use serenity::model::gateway::ActivityType::Playing;
use specs::prelude::*;
use specs::{World, WorldExt, Builder};
use specs::world::Index as EntityId;
use specs;
use specs::shred::FetchMut;


#[derive(Clone, Debug)]
pub struct PlayerInput {
    player_id: EntityId,
    input: String,
}

pub struct Game<'a, 'b> {
    world: World,
    dispatcher: specs::Dispatcher<'a, 'b>,
}

impl Game<'_, '_> {
    pub fn new<'a, 'b>() -> Game<'a, 'b> {
        let mut world = World::new();
        let mut dispatcher = specs::DispatcherBuilder::new()
            // .with(systems::RoomSystem, "room", &[])
            // .with(systems::RoomDescriptionSystem, "room_description", &[])
            .with(systems::PlayerSystem, "player", &[])
            .build();

        dispatcher.setup(&mut world);

        // let mut rooms: Vec<Entity> = Vec::new();
        // for _ in 0..20 {
        //     rooms.push(
        //         world.create_entity()
        //             .with(components::Room::new(None, None))
        //             .with(components::Description {
        //                 description: "The interior of the traincar smelled musty, like unwashed \
        //                 damp rags. Along the left wall were metal cabinets. Along the right wall \
        //                 were windows, dark glass framing the void outside the train.".to_string(),
        //                 glance: "".to_string(),
        //                 name: None,
        //             })
        //             .build()
        //     )
        // }

        // {  // Create separate block so room_storage goes out of scope
        //     let mut room_storage = world.write_storage::<components::Room>();
        //     for i in 0..20 {
        //         let room: &mut components::Room = room_storage.get_mut(rooms[i]).unwrap();
        //
        //         if i > 0 {
        //             room.set_previous_room(&rooms[i - 1]);
        //         }
        //
        //         if i < 19 {
        //             room.set_next_room(&rooms[i + 1]);
        //         }
        //     }
        // }

        let world_map = Game::build_map();

        world.create_entity()
            .with(components::Description {
                description: "It looks like a sturdy table!".to_string(),
                glance: "table".to_string(),
                name: None,
            })
            .with(components::InRoom { room: world_map.spawn() })
            .build();
        world.create_entity()
            .with(components::Description {
                description: "It looks like a metal cabinet.".to_string(),
                glance: "cabinet".to_string(),
                name: None,
            })
            .with(components::InRoom { room: world_map.spawn() })
            .build();
        world.create_entity()
            .with(components::Description {
                description: "A double reed woodwind instrument.".to_string(),
                glance: "oboe".to_string(),
                name: None,
            })
            .with(components::InRoom { room: world_map.spawn() })
            .build();

        world.insert(world_map);

        Game {
            world,
            dispatcher,
        }
    }

    fn spawn(&self) -> map::RoomId {
        let world_map: &map::Map = &*self.world.fetch::<map::Map>();
        world_map.spawn()
    }

    pub fn build_map() -> map::Map {
        let mut map = map::Map::new();
        let mut first_room = map.create_room("This is the first room");
        let mut second_room = map.create_room("This is the second room");
        second_room
            .add_exit(map::ExitDirection::Previous, first_room.as_gate("Hallway leading to the previous train car."));
        first_room
            .add_exit(map::ExitDirection::Next, second_room.as_gate("Hallway leading to the next train car."));

        map.set_spawn(first_room.id());
        map.rooms.insert(first_room.id(), first_room);
        map.rooms.insert(second_room.id(), second_room);

        map
    }

    pub fn tick(&mut self) {
        println!("=============== before tick ===============");
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
        println!("=============== after tick ===============");
    }

    pub fn create_player(&mut self) -> EntityId {
        let mut spawn: map::RoomId;
        {
            spawn = self.spawn()
        }
        println!("Creating player entity");
        let player = self.world.create_entity()
            .with(components::Player{})
            .with(components::InRoom { room: spawn })
            .build();
        println!("About to return player id");
        player.id()
    }

    pub fn player_input(&mut self, player_id: EntityId, input: &str) {
        let p: PlayerInput = PlayerInput {
            player_id,
            input: input.into(),
        };

        let mut list: Vec<PlayerInput>;
        {
            match self.world.try_fetch::<Vec<PlayerInput>>() {
                Some(l) => list = (*l).clone(),
                None => list = Vec::new(),
            }
        }

        list.push(p);
        self.world.insert(list);

        // match self.world.try_fetch() {
        //     Some(l) => {
        //         let mut list: Vec<PlayerInput> = *l.clone();
        //         list.push(p);
        //         self.world.insert(list);
        //     }
        //     None => {
        //         let mut l = Vec::new();
        //         l.push(p);
        //         self.world.insert(l);
        //     }
        // }
    }
}
