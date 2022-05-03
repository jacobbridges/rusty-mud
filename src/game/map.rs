use specs::prelude::*;
use std::collections::{BTreeMap, HashMap};
use specs::world::EntitiesRes;
use crate::game::components;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExitDirection {
    Next,
    Previous,
}

pub type RoomId = u64;

pub struct Gate {
    pub description: String,
    is_locked: bool,
    keys: Option<Vec<Entity>>,
    to: RoomId,
}

impl Gate {
    pub fn new(to: RoomId, description: &str) -> Self {
        Gate {
            description: description.to_string(),
            to,
            is_locked: false,
            keys: None,
        }
    }

    pub fn new_locked(to: RoomId, description: &str, keys: Vec<Entity>) -> Self {
        Gate {
            description: description.to_string(),
            to,
            keys: Some(keys),
            is_locked: true,
        }
    }

    pub fn add_key(&mut self, key: Entity) {
        let keys = self.keys.as_mut().unwrap();
        keys.push(key);
        self.keys = Some(keys.to_vec());
    }

    pub fn try_unlock(&mut self, key: Entity) {
        let keys = self.keys.as_ref().unwrap();
        if keys.contains(&key) {
            self.is_locked = false;
        }
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked.clone()
    }

    pub fn to(&self) -> RoomId {
        self.to.clone()
    }
}

pub struct Room {
    id: RoomId,
    description: String,
    pub exits: HashMap<ExitDirection, Gate>,
}

impl<'a> Room {
    pub fn new(id: RoomId, description: &str) -> Self {
        Room {
            id,
            description: description.to_string(),
            exits: HashMap::new(),
        }
    }

    pub fn id(&self) -> RoomId { self.id.clone() }

    pub fn add_exit(&mut self, dir: ExitDirection, gate: Gate) -> &mut Self {
        self.exits.insert(dir, gate);
        self
    }

    pub fn exit(&self, dir: ExitDirection) -> Option<&Gate> {
        self.exits.get(&dir)
    }

    pub fn as_gate(&self, description: &str) -> Gate {
        Gate::new(self.id.clone(), description)
    }

    pub fn as_gate_locked(&self, description: &str, keys: Vec<Entity>) -> Gate {
        Gate::new_locked(self.id.clone(), description, keys)
    }

    fn generate_room_description(&self, glances: &Vec<&str>) -> String {
        let mut final_str;
        let desc = self.description.clone();
        let mut split = desc.split("===");
        final_str = split.next().unwrap().trim().to_string();
        let mut obj_strings = glances.iter()
            .map(|glance| {
                match glance.chars().next() {
                    Some(c) => match c {
                        'a' | 'e' | 'i' | 'o' | 'u' => format!("an {}", glance).to_string(),
                        _ => format!("a {}", glance).to_string(),
                    },
                    None => "".to_string(),
                }
            })
            .collect::<Vec<String>>();
        obj_strings.retain(|x| x.len() > 0);

        if obj_strings.len() > 0 {
            final_str = format!("{}\n===\nYou see ", &final_str);
        }

        for i in 0..obj_strings.len() {
            if i == (obj_strings.len() - 1) {
                if i != 0 {
                    final_str = format!("{}, and ", &final_str);
                }
            } else if i > 0 {
                final_str = format!("{}, ", &final_str);
            }

            final_str = format!("{}{}", &final_str, &obj_strings[i]);

            if i == (obj_strings.len() - 1) {
                final_str = format!("{}.", &final_str);
            }
        }
        final_str
    }

    pub fn description(&mut self, entities: &EntitiesRes, inrooms: &mut WriteStorage<components::InRoom>, ds: &ReadStorage<components::Description>) -> String {
        let mut obj_glances: Vec<&str> = Vec::new();
        for (e, inroom) in (entities, inrooms).join() {
            if inroom.room == self.id {
                match ds.get(e) {
                    Some(d) => obj_glances.push(&d.glance),
                    None => {},
                }
            }
        }
        self.generate_room_description(&obj_glances)
    }
}


pub struct Map {
    pub rooms: BTreeMap<RoomId, Room>,
    room_indexer: u64,
    spawn: Option<RoomId>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            rooms: BTreeMap::new(),
            room_indexer: 0,
            spawn: None,
        }
    }

    pub fn create_room(&mut self, description: &str) -> Room {
        let room_id = self.room_indexer.clone();
        self.room_indexer += 1;
        Room::new(room_id.clone(), description)
    }

    pub fn set_spawn(&mut self, room_id: RoomId) {
        self.spawn = Some(room_id);
    }

    pub fn spawn(&self) -> RoomId {
        self.spawn.expect("No spawn point set for map!")
    }

    pub fn room(&mut self, room_id: &RoomId) -> &mut Room {
        self.rooms.get_mut(room_id).unwrap()
    }
}