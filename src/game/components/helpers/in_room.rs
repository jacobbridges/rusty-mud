use specs::prelude::*;
use crate::game::map;
use super::super::InRoom;

pub fn get_entity_room_id(entity: Entity, ecs: &World) -> Option<map::RoomId> {
    let inrooms = ecs.read_storage::<InRoom>();

    match inrooms.get(entity) {
        Some(c) => Some(c.room),
        None => None,
    }
}

pub fn get_room_entities_as_bitset(room_id: map::RoomId, ecs: &World) -> BitSet {
    let inrooms = ecs.read_storage::<InRoom>();
    let mut bitset = BitSet::new();

    for (entity, inroom) in (&ecs.entities(), &inrooms).join() {
        if inroom.room == room_id {
            bitset.add(entity.id());
        }
    }

    return bitset;
}
