use once_cell::sync::OnceCell;
use std::process::Command;
use std::sync::{Mutex, MutexGuard};

pub fn apply_layouts(joined_layouts: MutexGuard<&str>) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("setxkbmap -layout {}", joined_layouts))
        .output()
        .expect("failed to execute process");
}

pub fn get_current_layout_index(current_layout: &str, layouts: &Vec<&str>) -> usize {
    layouts
        .iter()
        .position(|&r| *r.to_string() == *current_layout.to_string())
        .unwrap()
}
