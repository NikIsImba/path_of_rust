// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod skill_tree;
mod ts_interface;


use skill_tree::{data::PathOfExileSkillTree, graph::SkillGraph};
use ts_interface::{get_base_size, get_group_locations, get_nodes_for_group};



fn main() {
    tauri::Builder::default()
        .manage(generate_skill_tree_from_json())
        .invoke_handler(tauri::generate_handler![get_base_size, get_group_locations, get_nodes_for_group])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct PoeState {
    skill_data: PathOfExileSkillTree,
    skill_graph: SkillGraph,
}

fn generate_skill_tree_from_json() -> PoeState {
    let skill_tree_json_string = include_str!("./data/skill_tree.json");

    let skill_data: PathOfExileSkillTree = match serde_json::from_str(skill_tree_json_string) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing skill tree JSON: {}", e);
            std::process::exit(1);
        }
    };

    let skill_graph = SkillGraph::generate_from_tree(&skill_data);

    PoeState {
        skill_data,
        skill_graph,
    }
}
