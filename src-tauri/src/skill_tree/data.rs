use serde::Deserialize;
use std::collections::HashMap;

use super::{
    node::Node,
    parsing::{
        from_string_array_to_u32_box, from_string_key_to_u32_with_group,
        from_string_key_to_u32_with_node,
    },
};

#[derive(Deserialize)]
pub struct PathOfExileSkillTree {
    tree: String,
    #[serde(deserialize_with = "from_string_key_to_u32_with_group")]
    groups: HashMap<u32, Group>,
    #[serde(deserialize_with = "from_string_key_to_u32_with_node")]
    nodes: HashMap<u32, Node>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl PathOfExileSkillTree {
    pub fn get_base_size(&self) -> (i32, i32) {
        (self.max_x - self.min_x, self.max_y - self.min_y)
    }

    pub fn get_group_locations(&self) -> HashMap<u32, (f32, f32)> {
        self.groups
            .iter()
            .map(|(k, v)| {
                let adjusted_x = v.x - self.min_x as f32;
                let adjusted_y = v.y - self.min_y as f32;
                (*k, (adjusted_x, adjusted_y))
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub struct Group {
    x: f32,
    y: f32,
    orbits: Box<[u8]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box")]
    nodes: Box<[u32]>,
}
