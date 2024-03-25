use crate::PoeState;

// This file is used to define the interface between the Rust backend and the TypeScript frontend.

#[derive(serde::Serialize)]
pub struct TsBaseSize{
    width: i32,
    height: i32,
}

#[tauri::command]
pub fn get_base_size(poe_state: tauri::State<PoeState>) -> TsBaseSize {
    let (width, height) = poe_state.skill_data.get_base_size();
    TsBaseSize{
        width,
        height,
    }
}

#[derive(serde::Serialize)]
pub struct TsGroupLocation{
    group_id: u32,
    x: f32,
    y: f32,
}

#[tauri::command]
pub fn get_group_locations(poe_state: tauri::State<PoeState>) -> Vec<TsGroupLocation> {
    let group_locations = poe_state.skill_data.get_group_locations();
    let mut group_locations_vec = Vec::new();
    for (group_id, (x, y)) in group_locations.iter(){
        group_locations_vec.push(TsGroupLocation{
            group_id: *group_id,
            x: *x,
            y: *y,
        });
    }
    group_locations_vec
}

#[derive(serde::Serialize)]
pub struct TsNode{
    node_id: u32,
    node_name: String,
    x: f32,
    y: f32,
}

#[tauri::command]
pub fn get_nodes_for_group(poe_state: tauri::State<PoeState>, group_id: u32) -> Vec<TsNode> {
    let group = match poe_state.skill_data.groups.get(&group_id){
        Some(group) => group,
        None => return Vec::new(),
    };

    let mut nodes = Vec::new();

    for node_id in group.nodes.iter(){
        let node = match poe_state.skill_data.nodes.get(node_id){
            Some(node) => node,
            None => continue,
        };

        let position_data = match poe_state.skill_graph.nodes.get(node_id){
            Some(position_data) => position_data,
            None => continue,
        };

        nodes.push(TsNode{
            node_id: *node_id,
            node_name: node.get_name().to_string(),
            x: position_data.x,
            y: position_data.y,
        });
    }

    nodes
}