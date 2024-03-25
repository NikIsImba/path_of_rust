use serde::Deserialize;
use std::{collections::HashMap, f32::consts::PI};

use super::{
    node::Node,
    parsing::common::{
        from_string_array_to_u32_box, from_string_key_to_u32_with_group,
        from_string_key_to_u32_with_node,
    },
};

#[derive(Deserialize)]
pub struct PathOfExileSkillTree {
    tree: String,
    #[serde(deserialize_with = "from_string_key_to_u32_with_group")]
    pub groups: HashMap<u32, Group>,
    #[serde(deserialize_with = "from_string_key_to_u32_with_node")]
    pub nodes: HashMap<u32, Node>,
    pub min_x: i32,
    pub min_y: i32,
    max_x: i32,
    max_y: i32,
    constants: Constants,
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

    pub fn calculate_orbits(&self) -> Box<[Box<[(f32, f32)]>]> {

        let mut orbits = Vec::new();

        for (orbit_index, skills_in_orbit) in self.constants.skills_per_orbit.iter().enumerate() {
            let radians_per_index = (2.0 * PI) / *skills_in_orbit as f32;
            let radius = self.constants.orbit_radii[orbit_index] as f32;
            let result = (0..*skills_in_orbit)
                .map(|index| {
                    let x = radius * radians_per_index.sin() * index as f32;
                    let y = radius * radians_per_index.cos() * index as f32;
            
                    (x, y)
                })
                .collect::<Vec<(f32, f32)>>();

                orbits.push(result.into_boxed_slice());
        }         

        orbits.into_boxed_slice()
        
    }
}

#[derive(Deserialize)]
pub struct Group {
    pub x: f32,
    pub y: f32,
    orbits: Box<[u8]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box")]
    pub nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct Constants{
    classes: HashMap<String, u32>,
    #[serde(rename = "characterAttributes")]
    character_attributes: HashMap<String, u32>,
    #[serde(rename = "PSSCentreInnerRadius")]
    psscentre_inner_radius: u32,
    #[serde(rename = "skillsPerOrbit")]
    skills_per_orbit: Box<[u32]>,
    #[serde(rename = "orbitRadii")]
    orbit_radii: Box<[u32]>,

}


