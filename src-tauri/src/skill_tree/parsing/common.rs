use std::collections::HashMap;
use serde::de::{Deserialize, Error};

use crate::skill_tree::{data::Group, node::Node};


pub fn from_string_key_to_u32_with_group<'de, D>(
    deserializer: D,
) -> Result<HashMap<u32, Group>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, Group> = HashMap::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse map"))?;

    map.into_iter()
        .map(|(k, v)| {
            let k = k.parse::<u32>().map_err(|_| D::Error::custom("Failed to parse key"))?;
            Ok((k, v))
        })
        .collect()
}

pub fn from_string_key_to_u32_with_node<'de, D>(
    deserializer: D,
) -> Result<HashMap<u32, Node>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, Node> = HashMap::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse map"))?;

    map.into_iter()
        .map(|(k, v)| {
            let k = match k.parse::<u32>() {
                Ok(k) => k,
                Err(_) => {
                    if k == "root" {
                        0
                    } else {
                        return Err(D::Error::custom("Failed to parse key"));
                    }
                }
            };
            Ok((k, v))
        })
        .collect()
}

pub fn from_string_array_to_u32_box<'de, D>(deserializer: D) -> Result<Box<[u32]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: Vec<String> = Vec::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse string"))?;

    string
        .into_iter()
        .map(|s| s.parse::<u32>().map_err(|_| D::Error::custom("Failed to parse string")))
        .collect()
}

pub fn from_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = String::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse string"))?;
 
    string.parse::<u32>()
        .map_err(|_| D::Error::custom("Failed to parse string"))
}

pub fn from_string_to_u32_option<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = String::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse string"))?;

    if string.is_empty() {
        Ok(None)
    } else {
        string.parse::<u32>()
            .map(Some)
            .map_err(|_| D::Error::custom("Failed to parse string"))
    }
}

pub fn from_string_array_to_box<'de, D>(deserializer: D) -> Result<Box<Box<str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: Vec<String> = Vec::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse string"))?;

    let result = string.into_iter().collect::<String>().into_boxed_str();

    Ok(Box::new(result))
}

pub fn from_string_array_to_box_option<'de, D>(
    deserializer: D,
) -> Result<Option<Box<Box<str>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: Vec<String> = Vec::deserialize(deserializer)
        .map_err(|_| D::Error::custom("Failed to parse string"))?;

    if string.is_empty() {
        Ok(None)
    } else {
        let result = string.into_iter().collect::<String>().into_boxed_str();
        Ok(Some(Box::new(result)))
    }
}

pub fn default_box_box_str_none() -> Option<Box<Box<str>>> {
    None
}

pub fn defualt_u32_none() -> Option<u32> {
    None
}

pub fn default_bool() -> bool {
    false
}

pub fn default_u8() -> u8 {
    0
}
