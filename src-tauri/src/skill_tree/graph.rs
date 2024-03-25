use std::collections::HashMap;

use super::data::PathOfExileSkillTree;

pub struct SkillGraph{
  pub nodes: HashMap<u32, GraphNode>,
  pub edges: Box<[GraphEdge]>,
}

impl SkillGraph{
  pub fn generate_from_tree(tree: &PathOfExileSkillTree) -> SkillGraph{
    let mut nodes = HashMap::new();
    let mut vec = Vec::new();

    let orbits = tree.calculate_orbits();

    for (_, group) in tree.groups.iter(){
      for node_id in group.nodes.iter(){
        let node = match tree.nodes.get(node_id){
          Some(node) => node,
          None => panic!("Node not found in tree.nodes"),
        };

        match node.get_position_data(){
          Ok(position_data) => {
            let offset = orbits[position_data.orbit as usize][position_data.orbit_index as usize];
            nodes.insert(*node_id, GraphNode{
              x: offset.0,
              y: offset.1,
            });
          },
          Err(_) => {
            nodes.insert(*node_id, GraphNode{
              x: 0 as f32,
              y: 0 as f32,
            });
          },
        }
        
      }
    }

    SkillGraph{
      nodes: nodes,
      edges: vec.into_boxed_slice(),
    }
  }
}

pub struct GraphNode{
  pub x: f32,
  pub y: f32,
}

pub struct GraphEdge{
  from: u32,
  to: u32,
}