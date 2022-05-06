use specs::prelude::*;
use super::super::Storage;

pub fn get_entities_in_storage_as_bitset(entity: Entity, ecs: &World) -> Option<BitSet> {
    let storages = ecs.read_storage::<Storage>();

    match storages.get(entity) {
        Some(store) => {
            let mut bitset = BitSet::new();
            for item in &store.items {
                bitset.add(item.id());
            }
            return Some(bitset);
        }
        None => None
    }
}