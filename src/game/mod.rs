mod components;
mod systems;
mod map;
mod player;

use specs::{World, WorldExt, Builder};
use specs::world::Index as EntityId;
use specs;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    NpcTurn,
}

pub struct Game<'a, 'b> {
    world: World,
    dispatcher: specs::Dispatcher<'a, 'b>,
    player_id: EntityId,
}

impl Game<'_, '_> {
    /// Create a new game object.
    pub fn new<'a, 'b>() -> Game<'a, 'b> {
        let mut world = World::new();
        let mut dispatcher = specs::DispatcherBuilder::new()
            .with(systems::MovementSystem, "movement", &[])
            .with(systems::InventorySystem, "inventory", &[])
            .build();

        // Any components not mentioned in systems must be manually mentioned here
        world.register::<components::Player>();
        world.register::<components::Item>();

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
            .with(components::Storage::new())
            .build();
        world.create_entity()
            .with(components::Description {
                description: "A double reed woodwind instrument.".to_string(),
                glance: "oboe".to_string(),
                name: None,
            })
            .with(components::InRoom { room: world_map.spawn() })
            .with(components::Item{})
            .build();

        let player = world.create_entity()
            .with(components::Player{})
            .with(components::InRoom { room: world_map.spawn() })
            .with(components::Storage::new())
            .build();

        world.insert(world_map);
        world.insert(RunState::PreRun);

        Game {
            world,
            dispatcher,
            player_id: player.id(),
        }
    }

    /// Get the world spawn point.
    fn spawn(&self) -> map::RoomId {
        let world_map: &map::Map = &*self.world.fetch::<map::Map>();
        world_map.spawn()
    }

    /// Build the default map.
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

    /// Game tick
    pub fn tick(&mut self) {
        let mut newrunstate: RunState;
        {
            let runstate = self.world.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::PreRun => {
                println!("=============== prerun tick ===============");
                self.dispatcher.dispatch(&self.world);
                self.world.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                println!("=============== input  tick ===============");
                newrunstate = player::handle_player_input(self);
            }
            RunState::PlayerTurn => {
                println!("=============== player tick ===============");
                self.dispatcher.dispatch(&self.world);
                self.world.maintain();
                newrunstate = RunState::NpcTurn;
            }
            RunState::NpcTurn => {
                println!("=============== NPC    tick ===============");
                self.dispatcher.dispatch(&self.world);
                self.world.maintain();
                newrunstate = RunState::AwaitingInput;
            }
        }

        {
            let mut runwriter = self.world.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
    }

    /// Create a player entity, inject into the world, and return the player entity id.
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

    /// Collect player input and inject into the ECS
    pub fn player_input(&mut self, player_id: EntityId, input: &str) {
        let p: player::PlayerInput = player::PlayerInput {
            player_id,
            input: input.into(),
        };

        let mut list: Vec<player::PlayerInput>;
        {
            match self.world.try_fetch::<Vec<player::PlayerInput>>() {
                Some(l) => list = (*l).clone(),
                None => list = Vec::new(),
            }
        }

        list.push(p);
        self.world.insert(list);
    }
}
