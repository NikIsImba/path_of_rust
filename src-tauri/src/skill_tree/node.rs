#![allow(unused)]
use std::collections::HashSet;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

use super::parsing::{
    default_bool, default_box_box_str_none, default_u8, defualt_u32_none, from_string_array_to_box,
    from_string_array_to_box_option, from_string_array_to_u32_box, from_string_to_u32,
    from_string_to_u32_option,
};

//@TODO: Continue to split up data like PositionData and AscendancyNode into their own structs

pub enum Node {
    Ascendancy(AscendancyNode, PositionData),
    ClassStart(ClassStartNode),
    Mastery(MasteryNode),
    Proxy(ProxyNode),
    DeprecatedMastery(DeprecatedMasteryNode),
    Notable(NotableNode),
    Keystone(KeystoneNode),
    StandaloneNotable(StandaloneNotableNode),
    StandaloneKeystone(StandaloneKeystoneNode),
    JewelSocket(JewelSocketNode),
    ExpansionJewelSocket(ExpansionJewelSocketNode),
    Normal(NormalNode),
    StandaloneNormal(StandaloneNormalNode),
    Root(PositionData),
}

#[derive(Deserialize)]
pub struct PositionData {
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
pub struct ProxyNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
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
pub struct DeprecatedMasteryNode {
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
pub struct KeystoneNode {
    skill: u32,
    name: Box<str>,
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
    icon: Box<str>,
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

#[derive(Deserialize)]
pub struct StandaloneKeystoneNode {
    skill: u32,
    name: Box<str>,
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
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
}

#[derive(Deserialize)]
pub struct JewelSocketNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
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
pub struct ExpansionJewelSocketNode {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
    #[serde(rename = "expansionJewel")]
    expansion_jewel: ExpansionJewelData,
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
pub struct ExpansionJewelData {
    size: u8,
    index: u8,
    #[serde(deserialize_with = "from_string_to_u32")]
    proxy: u32,
    #[serde(
        deserialize_with = "from_string_to_u32_option",
        default = "defualt_u32_none"
    )]
    parent: Option<u32>,
}

#[derive(Deserialize)]
pub struct NormalNode {
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
pub struct StandaloneNormalNode {
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
            match Deserialize::deserialize(value.clone()) {
                Ok(ascendancy_node) => match Deserialize::deserialize(value) {
                    Ok(position_data) => {
                        return Ok(Node::Ascendancy(ascendancy_node, position_data))
                    }
                    Err(e) => panic!("Failed to parse position data: {}", e),
                },
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
                    Ok(deprecated_mastery_node) => {
                        return Ok(Node::DeprecatedMastery(deprecated_mastery_node))
                    }
                    Err(e) => panic!("Failed to parse depricated mastery node: {}", e),
                },
            }
        }

        if map.contains_key("isProxy") {
            match Deserialize::deserialize(value.clone()) {
                Ok(proxy_node) => return Ok(Node::Proxy(proxy_node)),
                Err(e) => panic!("Failed to parse proxy node: {}", e),
            }
        }

        if map.contains_key("isNotable") {
            match Deserialize::deserialize(value.clone()) {
                Ok(notable_node) => return Ok(Node::Notable(notable_node)),
                Err(e) => match Deserialize::deserialize(value) {
                    Ok(standalone_notable_node) => {
                        return Ok(Node::StandaloneNotable(standalone_notable_node))
                    }
                    Err(e) => panic!("Failed to parse standalone notable node: {}", e),
                },
            }
        }

        if map.contains_key("isKeystone") {
            match Deserialize::deserialize(value.clone()) {
                Ok(keystone_node) => return Ok(Node::Keystone(keystone_node)),
                Err(e) => match Deserialize::deserialize(value) {
                    Ok(standalone_keystone_node) => {
                        return Ok(Node::StandaloneKeystone(standalone_keystone_node))
                    }
                    Err(e) => panic!("Failed to parse standalone keystone node: {}", e),
                },
            }
        }

        if map.contains_key("isJewelSocket") {
            if (map.contains_key("expansionJewel")) {
                match Deserialize::deserialize(value) {
                    Ok(expansion_jewel_socket_node) => {
                        return Ok(Node::ExpansionJewelSocket(expansion_jewel_socket_node))
                    }
                    Err(e) => panic!("Failed to parse expansion jewel socket node: {}", e),
                }
            }

            match Deserialize::deserialize(value) {
                Ok(jewel_socket_node) => return Ok(Node::JewelSocket(jewel_socket_node)),
                Err(e) => panic!("Failed to parse jewel socket node: {}", e),
            }
        }

        if map.contains_key("skill") {
            match Deserialize::deserialize(value.clone()) {
                Ok(normal_node) => return Ok(Node::Normal(normal_node)),
                Err(e) => match Deserialize::deserialize(value) {
                    Ok(standalone_normal_node) => {
                        return Ok(Node::StandaloneNormal(standalone_normal_node))
                    }
                    Err(e) => panic!("Failed to parse standalone normal node: {}", e),
                },
            }
        }

        match Deserialize::deserialize(value) {
            Ok(root_node) => Ok(Node::Root(root_node)),
            Err(e) => panic!("Failed to parse root node: {}", e),
        }
    }
}
