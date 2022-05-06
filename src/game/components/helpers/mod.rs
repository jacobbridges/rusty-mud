mod in_room;
mod storage;

pub use in_room::{
    get_entity_room_id,
    get_room_entities_as_bitset,
};
pub use storage::{
    get_entities_in_storage_as_bitset,
};