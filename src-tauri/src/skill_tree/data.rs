use serde::Deserialize;
use std::{collections::HashMap, vec};

#[derive(Deserialize)]
pub struct PathOfExileSkillTree {
    pub tree: String,
    #[serde(deserialize_with = "from_string_key_to_i32")]
    pub groups: HashMap<i32, Group>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl PathOfExileSkillTree {
    pub fn get_base_size(&self) -> (i32, i32) {
        (self.max_x - self.min_x, self.max_y - self.min_y)
    }

    pub fn get_group_locations(&self) -> HashMap<i32, (f32, f32)> {
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
    pub x: f32,
    pub y: f32,
    pub orbits: Box<[u8]>,
    #[serde(deserialize_with = "from_string_to_u32")]
    pub nodes: Box<[u32]>,
}

fn from_string_key_to_i32<'de, D>(deserializer: D) -> Result<HashMap<i32, Group>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, Group> = match HashMap::deserialize(deserializer) {
        Ok(map) => map,
        Err(e) => panic!("Failed to parse map: {}", e),
    };

    map.into_iter()
        .map(|(k, v)| {
            let k = match k.parse::<i32>() {
                Ok(k) => k,
                Err(e) => panic!("Failed to parse key: {}", e),
            };
            Ok((k, v))
        })
        .collect()
}

fn from_string_to_u32<'de, D>(deserializer: D) -> Result<Box<[u32]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: vec::Vec<String> = match vec::Vec::deserialize(deserializer) {
        Ok(string) => string,
        Err(e) => panic!("Failed to parse string: {}", e),
    };

    string
        .into_iter()
        .map(|s| match s.parse::<u32>() {
            Ok(s) => Ok(s),
            Err(e) => panic!("Failed to parse string: {}", e),
        })
        .collect()
}
