use std::{collections::HashMap, vec};

use serde::Deserialize;

use super::{data::Group, node::Node};

pub fn from_string_key_to_u32_with_group<'de, D>(
    deserializer: D,
) -> Result<HashMap<u32, Group>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map: HashMap<String, Group> = match HashMap::deserialize(deserializer) {
        Ok(map) => map,
        Err(e) => panic!("Failed to parse map: {}", e),
    };

    map.into_iter()
        .map(|(k, v)| {
            let k = match k.parse::<u32>() {
                Ok(k) => k,
                Err(e) => panic!("Failed to parse key: {}", e),
            };
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
    let map: HashMap<String, Node> = match HashMap::deserialize(deserializer) {
        Ok(map) => map,
        Err(e) => panic!("Failed to parse map: {}", e),
    };

    map.into_iter()
        .map(|(k, v)| {
            let k = match k.parse::<u32>() {
                Ok(k) => k,
                Err(e) => {
                    if k == "root" {
                        return Ok((0, v));
                    }
                    panic!("Failed to parse key: {}", e)
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

pub fn from_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = match String::deserialize(deserializer) {
        Ok(string) => string,
        Err(e) => panic!("Failed to parse string: {}", e),
    };

    match string.parse::<u32>() {
        Ok(s) => Ok(s),
        Err(e) => panic!("Failed to parse string: {}", e),
    }
}

pub fn from_string_to_u32_option<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = match String::deserialize(deserializer) {
        Ok(string) => string,
        Err(e) => panic!("Failed to parse string: {}", e),
    };

    if string.is_empty() {
        return Ok(None);
    }

    match string.parse::<u32>() {
        Ok(s) => Ok(Some(s)),
        Err(e) => panic!("Failed to parse string: {}", e),
    }
}

pub fn from_string_array_to_box<'de, D>(deserializer: D) -> Result<Box<Box<str>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: vec::Vec<String> = match vec::Vec::deserialize(deserializer) {
        Ok(string) => string,
        Err(e) => panic!("Failed to parse string: {}", e),
    };

    Ok(Box::new(
        string.into_iter().collect::<String>().into_boxed_str(),
    ))
}

pub fn from_string_array_to_box_option<'de, D>(
    deserializer: D,
) -> Result<Option<Box<Box<str>>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: vec::Vec<String> = match vec::Vec::deserialize(deserializer) {
        Ok(string) => string,
        Err(e) => panic!("Failed to parse string: {}", e),
    };

    if string.is_empty() {
        return Ok(None);
    }

    Ok(Some(Box::new(
        string.into_iter().collect::<String>().into_boxed_str(),
    )))
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
