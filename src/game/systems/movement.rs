use specs::prelude::*;

use crate::game::map;
use crate::game::components;


pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadExpect<'a, map::Map>,
        WriteStorage<'a, components::ApplyMove>,
        WriteStorage<'a, components::InRoom>,
        ReadStorage<'a, components::Description>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            mut apply_moves,
            mut inrooms,
            descriptions,
            entities,
        ) = data;

        let mut new_room: Option<map::RoomId> = None;
        for (move_, inroom) in (&apply_moves, &mut inrooms).join() {
            inroom.room = move_.room.clone();
            new_room = Some(inroom.room.clone());
        }
        apply_moves.clear();

        // TODO(networking): Rework this for multiple players
        if let Some(id) = new_room {
            let room = map.room(&id);
            println!("{}", room.description(
                &entities,
                &mut inrooms,
                &descriptions,
            ));
        }
    }
}