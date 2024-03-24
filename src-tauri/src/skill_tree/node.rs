use serde::Deserialize;

use super::parsing::common::{
    default_bool, default_box_box_str_none, default_u8, defualt_u32_none, from_string_array_to_box,
    from_string_array_to_box_option, from_string_array_to_u32_box, from_string_to_u32,
    from_string_to_u32_option,
};

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