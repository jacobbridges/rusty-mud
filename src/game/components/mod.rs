// mod room;

use std::string::String;
use specs::{Component, VecStorage};
use crate::game::map;
// pub use room::Room;


#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Description {
    // Full description of the entity.
    pub description: String,
    // Description of the entity at a glance.
    pub glance: String,
    // Name of the entity, if the entity supports naming.
    pub name: Option<String>,
}

impl Description {
    pub fn new() -> Self {
        Description {
            description: "".into(),
            glance: "".into(),
            name: None,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct InRoom {
    pub room: map::RoomId,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Name;
