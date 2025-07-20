use std::{fs, path::PathBuf};

use crate::utils;

pub fn mass_update(dir: &PathBuf) {
    let files = fs::read_dir(dir).unwrap();
    let files = files.filter_map(|f| f.ok());

    for file in files {
        let path = file.path();
        if let Some(ext) = path.extension() {
            if ext == "desktop" {
                utils::update_desktop_entry(&path)
            }
        }
    }
}

pub fn run() {
    let dir = PathBuf::new();
    let dir = dir.join(utils::get_applications_dir());
    mass_update(&dir);
}
