//! Point d'entree Tauri pour spec-forge

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    spec_forge_app::run();
}
