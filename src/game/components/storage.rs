use specs::{Component, Entity, VecStorage};


#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Storage {
    pub items: Vec<Entity>,
}

impl Storage {
    pub fn new() -> Self {
        Storage { items: Vec::new() }
    }
}
