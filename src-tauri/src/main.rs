// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod skill_tree;

use std::collections::HashMap;

use skill_tree::data::PathOfExileSkillTree;

#[tauri::command]
fn get_base_size(tree: tauri::State<PathOfExileSkillTree>) -> (i32, i32) {
    tree.get_base_size()
}

#[tauri::command]
fn get_group_locations(tree: tauri::State<PathOfExileSkillTree>) -> HashMap<i32, (f32, f32)> {
    tree.get_group_locations()
}

fn main() {
    tauri::Builder::default()
        .manage(generate_skill_tree_from_json())
        .invoke_handler(tauri::generate_handler![get_base_size, get_group_locations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_skill_tree_from_json() -> PathOfExileSkillTree {
    let skill_tree_json_string = include_str!("./data/skill_tree.json");

    match serde_json::from_str(skill_tree_json_string) {
        Ok(skill_tree) => skill_tree,
        Err(e) => panic!("Failed to parse skill tree JSON: {}", e),
    }
}
