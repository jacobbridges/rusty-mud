use specs::prelude::*;

use crate::game::components;


pub struct InventorySystem;

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        WriteStorage<'a, components::ApplyInventoryChange>,
        WriteStorage<'a, components::Storage>,
        WriteStorage<'a, components::InRoom>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut changes,
            mut storages,
            mut inrooms,
            entities,
        ) = data;

        for (entity, change) in (&entities, &changes).join() {
            match (change.from_container, change.to_container) {
                (Some(from_container), Some(to_container)) => {
                    if storages.get(from_container).is_none() || storages.get(to_container).is_none() {
                        println!("Error: Failed to move item {} between containers because one \
                        or both containers are missing the Storage component.", entity.id());
                        return
                    }
                    let from_store = storages.get_mut(from_container).unwrap();
                    from_store.items.retain(|item| {*item != entity});
                    let to_store = storages.get_mut(to_container).unwrap();
                    to_store.items.push(entity);
                }
                (Some(from_container), None) => {
                    if inrooms.get(from_container).is_none() {
                        println!("Error: Failed to drop item {} because its container {} is not \
                        attached to any room.", entity.id(), from_container.id());
                        return
                    }
                    let inroom = inrooms.get(from_container)
                        .expect(format!("Failed to drop item {} because it already is attached to a room", entity.id()).as_str());
                    let room = inroom.room.clone();
                    let from_store = storages.get_mut(from_container)
                        .expect(format!("Expected entity {} to have a storage component", from_container.id()).as_str());
                    from_store.items.retain(|item| { *item != entity });
                    inrooms.insert(entity, components::InRoom { room })
                        .expect("Failed to insert InRoom component");
                }
                (None, Some(to_container)) => {
                    let _ = inrooms.get(entity)
                        .expect(format!("Failed to pick up item {} because it is not attached to a room", entity.id()).as_str());
                    let to_store = storages.get_mut(to_container)
                        .expect(format!("Expected entity {} to have a storage component", to_container.id()).as_str());
                    inrooms.remove(entity)
                        .expect("Failed to remove InRoom component");
                    to_store.items.push(entity);
                }
                (None, None) => {
                    println!("Error: WTF am I supposed to do with this case?");
                }
            }
        }

        changes.clear()
    }
}