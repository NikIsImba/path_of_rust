use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::skill_tree::node::Node;

const ASCENDANCY_FLAG: &str = "ascendancyName";
const CLASS_START_FLAG: &str = "classStartIndex";
const MASTERY_FLAG: &str = "isMastery";
const PROXY_FLAG: &str = "isProxy";
const NOTABLE_FLAG: &str = "isNotable";
const KEYSTONE_FLAG: &str = "isKeystone";
const JEWEL_SOCKET_FLAG: &str = "isJewelSocket";
const EXPANSION_JEWEL_FLAG: &str = "expansionJewel";
const SKILL_FLAG: &str = "skill";

impl<'de> Deserialize<'de> for Node {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
      let value: Value = Deserialize::deserialize(deserializer)?;
      let map = value.as_object().ok_or(serde::de::Error::custom("Expected object"))?;

      if map.contains_key(ASCENDANCY_FLAG) {
          let ascendancy_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse ascendancy node"))?;
          let main_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
          let position_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
          return Ok(Node::Ascendancy(main_data, position_data, ascendancy_node));
      }

      if map.contains_key(CLASS_START_FLAG) {
          let class_start_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse class start node"))?;
          let main_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
          let position_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
          return Ok(Node::ClassStart(main_data, position_data, class_start_node));
      }

      if map.contains_key(MASTERY_FLAG) {
          match Deserialize::deserialize(&value) {
              Ok(mastery_node) => {
                  let position_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
                  let main_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  return Ok(Node::Mastery(main_data, position_data, mastery_node))
              },
              Err(_) => {
                  let deprecated_mastery_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse deprecated mastery node"))?;
                  return Ok(Node::DeprecatedMastery(deprecated_mastery_node))
              },
          }
      }

      if map.contains_key(PROXY_FLAG) {
          let proxy_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse proxy node"))?;
          let position_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
          return Ok(Node::Proxy(proxy_node, position_data));
      }

      if map.contains_key(NOTABLE_FLAG) {
          match Deserialize::deserialize(&value) {
              Ok(position_data) => {
                  let main_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  let notable_node = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse notable node"))?;
                  return Ok(Node::Notable(main_data, position_data, notable_node))
              }
              Err(_) => {
                  let standalone_notable_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse standalone notable node"))?;
                  let main_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  return Ok(Node::StandaloneNotable(main_data, standalone_notable_node))
              }
          }
      }

      if map.contains_key(KEYSTONE_FLAG) {
          match Deserialize::deserialize(&value) {
              Ok(position_data) => {
                  let main_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  let keystone_node = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse keystone node"))?;
                  return Ok(Node::Keystone(main_data, position_data, keystone_node))
              }
              Err(_) => {
                  let standalone_keystone_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse standalone keystone node"))?;
                  let main_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  return Ok(Node::StandaloneKeystone(main_data, standalone_keystone_node))
              }
          }
      }

      if map.contains_key(JEWEL_SOCKET_FLAG) {
          if let Some(expansion_jewel_value) = map.get(EXPANSION_JEWEL_FLAG) {
              let expansion_jewel_node = Deserialize::deserialize(expansion_jewel_value).map_err(|_| serde::de::Error::custom("Failed to parse expansion jewel node"))?;
              let position_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
              let main_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
              return Ok(Node::ExpansionJewelSocket(main_data, position_data, expansion_jewel_node));
          }

          let jewel_socket_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse jewel socket node"))?;
          let position_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse position data"))?;
          return Ok(Node::JewelSocket(jewel_socket_node, position_data));
      }

      if map.contains_key(SKILL_FLAG) {
          match Deserialize::deserialize(&value) {
              Ok(position_data) => {
                  let main_data = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  let normal_node = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse normal node"))?;
                  return Ok(Node::Normal(main_data, position_data, normal_node))
              }
              Err(_) => {
                  let standalone_normal_node = Deserialize::deserialize(&value).map_err(|_| serde::de::Error::custom("Failed to parse standalone normal node"))?;
                  let main_data = Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse main data"))?;
                  return Ok(Node::StandaloneNormal(main_data, standalone_normal_node))
              }
          }
      }

      Ok(Node::Root(Deserialize::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to parse root node"))?))
  }
}