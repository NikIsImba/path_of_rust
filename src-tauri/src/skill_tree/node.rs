#![allow(unused)]
use std::collections::HashSet;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::parsing::{
    default_bool, default_box_box_str_none, default_u8, from_string_array_to_box,
    from_string_array_to_box_option, from_string_array_to_u32_box, from_string_to_u32,
};

pub enum Node {
    Ascendancy(AscendancyNode),
    ClassStart(ClassStartNode),
    Mastery(MasteryNode),
    DepricatedMastery(DepricatedMasteryNode),
    Notable(NotableNode),
    StandaloneNotable(StandaloneNotableNode),
    Normal,
    Root,
    DEBUG,
}

#[derive(Deserialize)]
pub struct RootNode {
    group: u32,
    orbit: u8,
    orbit_index: u8,
    #[serde(deserialize_with = "from_string_array_to_u32_box")]
    out_nodes: Box<[u32]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box")]
    in_nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct AscendancyNode {
    skill: u32,
    name: Box<str>,
    #[serde(rename = "isNotable", default = "default_bool")]
    is_notable: bool,
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(rename = "grantedStrength", default = "default_u8")]
    granted_strength: u8,
    #[serde(rename = "grantedDexterity", default = "default_u8")]
    granted_dexterity: u8,
    #[serde(rename = "grantedIntelligence", default = "default_u8")]
    granted_intelligence: u8,
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "flavourText"
    )]
    flavour_text: Option<Box<Box<str>>>,
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
    #[serde(rename = "isJewelSocket", default = "default_bool")]
    is_jewel_socket: bool,
    #[serde(rename = "ascendancyName")]
    ascendancy: Box<str>,
    #[serde(rename = "isAscendancyStart", default = "default_bool")]
    is_ascendancy_start: bool,
    #[serde(rename = "grantedPassivePoints", default = "default_u8")]
    granted_passive_points: u8,
    #[serde(rename = "isMultipleChoice", default = "default_bool")]
    is_multiple_choice: bool,
    #[serde(rename = "isMultipleChoiceOption", default = "default_bool")]
    is_multiple_choice_option: bool,
    group: u32,
    orbit: u8,
    #[serde(rename = "orbitIndex")]
    orbit_index: u8,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "out")]
    out_nodes: Box<[u32]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "in")]
    in_nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct ClassStartNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(rename = "classStartIndex")]
    class_start_index: u8,
    group: u32,
    orbit: u8,
    #[serde(rename = "orbitIndex")]
    orbit_index: u8,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "out")]
    out_nodes: Box<[u32]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "in")]
    in_nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct MasteryNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(rename = "inactiveIcon")]
    inactive_icon: Box<str>,
    #[serde(rename = "activeIcon")]
    active_icon: Box<str>,
    #[serde(rename = "activeEffectImage")]
    active_effect_image: Box<str>,
    #[serde(rename = "masteryEffects")]
    mastery_effects: Box<[MasteryEffect]>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    group: u32,
    orbit: u8,
    #[serde(rename = "orbitIndex")]
    orbit_index: u8,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "out")]
    out_nodes: Box<[u32]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "in")]
    in_nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct MasteryEffect {
    effect: u32,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
}

#[derive(Deserialize)]
pub struct DepricatedMasteryNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
}

#[derive(Deserialize)]
pub struct NotableNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    recipe: Box<Box<str>>,
    #[serde(rename = "isBlighted", default = "default_bool")]
    is_blighted: bool,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(rename = "grantedStrength", default = "default_u8")]
    granted_strength: u8,
    #[serde(rename = "grantedDexterity", default = "default_u8")]
    granted_dexterity: u8,
    #[serde(rename = "grantedIntelligence", default = "default_u8")]
    granted_intelligence: u8,
    group: u32,
    orbit: u8,
    #[serde(rename = "orbitIndex")]
    orbit_index: u8,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "out")]
    out_nodes: Box<[u32]>,
    #[serde(deserialize_with = "from_string_array_to_u32_box", rename = "in")]
    in_nodes: Box<[u32]>,
}

#[derive(Deserialize)]
pub struct StandaloneNotableNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(rename = "grantedStrength", default = "default_u8")]
    granted_strength: u8,
    #[serde(rename = "grantedDexterity", default = "default_u8")]
    granted_dexterity: u8,
    #[serde(rename = "grantedIntelligence", default = "default_u8")]
    granted_intelligence: u8,
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let map = match value.as_object() {
            Some(map) => map,
            None => return Err(serde::de::Error::custom("Expected object")),
        };

        if map.contains_key("ascendancyName") {
            match Deserialize::deserialize(value) {
                Ok(ascendancy_node) => return Ok(Node::Ascendancy(ascendancy_node)),
                Err(e) => panic!("Failed to parse ascendancy node: {}", e),
            }
        }

        if map.contains_key("classStartIndex") {
            match Deserialize::deserialize(value) {
                Ok(class_start_node) => return Ok(Node::ClassStart(class_start_node)),
                Err(e) => panic!("Failed to parse class start node: {}", e),
            }
        }

        if map.contains_key("isMastery") {
            match Deserialize::deserialize(value.clone()) {
                Ok(mastery_node) => return Ok(Node::Mastery(mastery_node)),
                Err(e) => match Deserialize::deserialize(value) {
                    Ok(depricated_mastery_node) => {
                        return Ok(Node::DepricatedMastery(depricated_mastery_node))
                    }
                    Err(e) => panic!("Failed to parse depricated mastery node: {}", e),
                },
            }
        }

        if map.contains_key("isNotable") {
            match Deserialize::deserialize(value.clone()) {
                Ok(notable_node) => return Ok(Node::Notable(notable_node)),
                Err(e) => match Deserialize::deserialize(value.clone()) {
                    Ok(standalone_notable_node) => {
                        return Ok(Node::StandaloneNotable(standalone_notable_node))
                    }

                    Err(e) => panic!("Failed to parse standalone notable node: {}", e),
                },
            }
        }

        Ok(Node::DEBUG)
    }
}
