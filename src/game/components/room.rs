use specs::{Component, Entity, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Room {
    previous_room_id: Option<specs::world::Index>,
    next_room_id: Option<specs::world::Index>,
}

impl Room {
    pub fn new(previous_room_id: Option<specs::world::Index>, next_room_id: Option<specs::world::Index>) -> Self {
        Room {
            previous_room_id,
            next_room_id,
        }
    }

    pub fn set_next_room(&mut self, room: &Entity) {
        self.next_room_id = Some(room.id())
    }

    pub fn get_next_room_id(&self) -> Option<specs::world::Index> {
        self.next_room_id.clone()
    }

    pub fn set_previous_room(&mut self, room: &Entity) {
        self.previous_room_id = Some(room.id())
    }

    pub fn get_previous_room_id(&self) -> Option<specs::world::Index> {
        self.previous_room_id.clone()
    }
}