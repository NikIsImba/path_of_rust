use serde::{Deserialize, Deserializer};
use serde_json::Value;

use super::parsing::{
    default_bool, default_box_box_str_none, default_u8, defualt_u32_none, from_string_array_to_box,
    from_string_array_to_box_option, from_string_array_to_u32_box, from_string_to_u32,
    from_string_to_u32_option,
};

const ASCENDANCY_FLAG: &str = "ascendancyName";
const CLASS_START_FLAG: &str = "classStartIndex";
const MASTERY_FLAG: &str = "isMastery";
const PROXY_FLAG: &str = "isProxy";
const NOTABLE_FLAG: &str = "isNotable";
const KEYSTONE_FLAG: &str = "isKeystone";
const JEWEL_SOCKET_FLAG: &str = "isJewelSocket";
const EXPANSION_JEWEL_FLAG: &str = "expansionJewel";
const SKILL_FLAG: &str = "skill";

pub enum Node {
    Normal(NodeMainData, NodePositionData, NormalNode),
    StandaloneNormal(NodeMainData, NormalNode),
    Notable(NodeMainData, NodePositionData, NotableNode),
    StandaloneNotable(NodeMainData, StandaloneNotableNode),
    Keystone(NodeMainData, NodePositionData, KeystoneNode),
    StandaloneKeystone(NodeMainData, KeystoneNode),
    Mastery(NodeMainData, NodePositionData, MasteryNode),
    JewelSocket(NodeMainData, NodePositionData),
    ExpansionJewelSocket(NodeMainData, NodePositionData, ExpansionJewelNode),
    Ascendancy(NodeMainData, NodePositionData, AscendancyNode),
    ClassStart(NodeMainData, NodePositionData, ClassStartNode),
    Proxy(NodeMainData, NodePositionData),
    Root(NodePositionData),
    DeprecatedMastery(NodeMainData),
}

#[derive(Deserialize)]
pub struct NodeMainData {
    skill: u32,
    name: Box<str>,
    icon: Box<str>,
    #[serde(deserialize_with = "from_string_array_to_box")]
    stats: Box<Box<str>>,
}
#[derive(Deserialize)]
pub struct NodePositionData {
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
pub struct Attribute {
    #[serde(rename = "grantedStrength", default = "default_u8")]
    granted_strength: u8,
    #[serde(rename = "grantedDexterity", default = "default_u8")]
    granted_dexterity: u8,
    #[serde(rename = "grantedIntelligence", default = "default_u8")]
    granted_intelligence: u8,
}

#[derive(Deserialize)]
pub struct AscendancyNode {
    #[serde(rename = "isNotable", default = "default_bool")]
    is_notable: bool,
    #[serde(flatten)]
    attribute: Attribute,
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
    #[serde(rename = "classStartIndex")]
    class_start_index: u8,
}

#[derive(Deserialize)]
pub struct MasteryNode {
    #[serde(rename = "inactiveIcon")]
    inactive_icon: Box<str>,
    #[serde(rename = "activeIcon")]
    active_icon: Box<str>,
    #[serde(rename = "activeEffectImage")]
    active_effect_image: Box<str>,
    #[serde(rename = "masteryEffects")]
    mastery_effects: Box<[MasteryEffect]>,
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
pub struct NotableNode {
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
    #[serde(flatten)]
    attribute: Attribute,
}

#[derive(Deserialize)]
pub struct KeystoneNode {
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
}

#[derive(Deserialize)]
pub struct StandaloneNotableNode {
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
    #[serde(flatten)]
    attribute: Attribute,
}

#[derive(Deserialize)]
pub struct ExpansionJewelNode {
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
    #[serde(
        deserialize_with = "from_string_array_to_box_option",
        default = "default_box_box_str_none",
        rename = "reminderText"
    )]
    reminder_text: Option<Box<Box<str>>>,
    #[serde(flatten)]
    attribute: Attribute,
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

        if map.contains_key(ASCENDANCY_FLAG) {
            let ascendancy_node = Deserialize::deserialize(&value).expect("Failed to parse ascendancy node");
            let main_data = Deserialize::deserialize(&value).expect("Failed to parse main data");
            let position_data = Deserialize::deserialize(value).expect("Failed to parse position data");
            return Ok(Node::Ascendancy(main_data, position_data, ascendancy_node));
        }

        if map.contains_key(CLASS_START_FLAG) {
            let class_start_node = Deserialize::deserialize(&value).expect("Failed to parse class start node");
            let main_data = Deserialize::deserialize(&value).expect("Failed to parse main data");
            let position_data = Deserialize::deserialize(value).expect("Failed to parse position data");
            return Ok(Node::ClassStart(main_data, position_data, class_start_node));
        }

        if map.contains_key(MASTERY_FLAG) {
            match Deserialize::deserialize(&value) {
                Ok(mastery_node) => {
                    let position_data = Deserialize::deserialize(&value).expect("Failed to parse position data");
                    let main_data = Deserialize::deserialize(value).expect("Failed to parse main data");
                    return Ok(Node::Mastery(main_data, position_data, mastery_node))
                },
                Err(_) => {
                    let deprecated_mastery_node = Deserialize::deserialize(&value).expect("Failed to parse deprecated mastery node");
                    return Ok(Node::DeprecatedMastery(deprecated_mastery_node))
                },
            }
        }

        if map.contains_key(PROXY_FLAG) {
            let proxy_node = Deserialize::deserialize(&value).expect("Failed to parse proxy node");
            let position_data = Deserialize::deserialize(value).expect("Failed to parse position data");
            return Ok(Node::Proxy(proxy_node, position_data));
        }

        if map.contains_key(NOTABLE_FLAG) {
            match Deserialize::deserialize(&value) {
                Ok(position_data) => {
                    let main_data = Deserialize::deserialize(&value).expect("Failed to parse main data");
                    let notable_node = Deserialize::deserialize(value).expect("Failed to parse notable node");
                    return Ok(Node::Notable(main_data, position_data, notable_node))
                }
                Err(_) => {
                    let standalone_notable_node = Deserialize::deserialize(&value).expect("Failed to parse standalone notable node");
                    let main_data = Deserialize::deserialize(value).expect("Failed to parse main data");
                    return Ok(Node::StandaloneNotable(main_data, standalone_notable_node))
                }
            }
        }

        if map.contains_key(KEYSTONE_FLAG) {
            match Deserialize::deserialize(&value) {
                Ok(position_data) => {
                    let main_data = Deserialize::deserialize(&value).expect("Failed to parse main data");
                    let keystone_node = Deserialize::deserialize(value).expect("Failed to parse keystone node");
                    return Ok(Node::Keystone(main_data, position_data, keystone_node))
                }
                Err(_) => {
                    let standalone_keystone_node = Deserialize::deserialize(&value).expect("Failed to parse standalone keystone node");
                    let main_data = Deserialize::deserialize(value).expect("Failed to parse main data");
                    return Ok(Node::StandaloneKeystone(main_data, standalone_keystone_node))
                }
            }
        }

        if map.contains_key(JEWEL_SOCKET_FLAG) {
            if let Some(expansion_jewel_value) = map.get(EXPANSION_JEWEL_FLAG) {
                let expansion_jewel_node = Deserialize::deserialize(expansion_jewel_value).expect("Failed to parse expansion jewel node");
                let position_data = Deserialize::deserialize(&value).expect("Failed to parse position data");
                let main_data = Deserialize::deserialize(value).expect("Failed to parse main data");
                return Ok(Node::ExpansionJewelSocket(main_data, position_data, expansion_jewel_node));
            }

            let jewel_socket_node = Deserialize::deserialize(&value).expect("Failed to parse jewel socket node");
            let position_data = Deserialize::deserialize(value).expect("Failed to parse position data");
            return Ok(Node::JewelSocket(jewel_socket_node, position_data));
        }

        if map.contains_key(SKILL_FLAG) {
            match Deserialize::deserialize(&value) {
                Ok(position_data) => {
                    let main_data = Deserialize::deserialize(&value).expect("Failed to parse main data");
                    let normal_node = Deserialize::deserialize(value).expect("Failed to parse normal node");
                    return Ok(Node::Normal(main_data, position_data, normal_node))
                }
                Err(_) => {
                    let standalone_normal_node = Deserialize::deserialize(&value).expect("Failed to parse standalone normal node");
                    let main_data = Deserialize::deserialize(value).expect("Failed to parse main data");
                    return Ok(Node::StandaloneNormal(main_data, standalone_normal_node))
                }
            }
        }

        return Ok(Node::Root(Deserialize::deserialize(value).expect("Failed to parse root node")));
    }
}
