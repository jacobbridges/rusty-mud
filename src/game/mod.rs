mod components;
mod systems;
mod map;

use specs::{World, WorldExt, Builder};
use specs::world::Index as EntityId;
use specs;


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
            .with(systems::PlayerInputSystem, "player_input", &[])
            .build();

        dispatcher.setup(&mut world);

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
        let spawn: map::RoomId;
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
    }
}
