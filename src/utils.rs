use inotify::{Inotify, WatchMask};
use std::fs::OpenOptions;
// TODO: Why I need to import this?
use std::io::Write;
use std::str::Split;
use std::{ffi::OsStr, fs, path::PathBuf};

use crate::constants;

pub fn watch_directory() -> Inotify {
    let inotify = Inotify::init().expect("Failed to initialize inotify");
    let mut watched_path = PathBuf::new();

    watched_path.push(constants::DESKTOP_PATH);

    inotify
        .watches()
        .add(watched_path, WatchMask::CREATE)
        .expect("Failed to add inotify watch");

    inotify
}

pub fn on_file_creation(raw_file: Option<&OsStr>) {
    let file_name = raw_file.unwrap();
    let mut file_path = PathBuf::new();

    file_path.push(constants::DESKTOP_PATH);
    file_path.push(file_name);

    let original_contents =
        fs::read_to_string(&file_path).expect("Should have been able to read the file");
    let contents = original_contents.split("\n");

    // TODO: improve it
    let steam_id = match find_steam_id(contents.clone()) {
        Ok(id) => id,
        Err(_) => "".to_string(),
    };

    let has_steam_id = steam_id != "";
    let has_startup_wm_class = has_startup_wm_class(contents.clone());

    if !has_startup_wm_class && has_steam_id {
        add_startup_wm_class(steam_id, file_path);
        cp_file_to_app_menu(file_name);
    } else {
        println!("The file do not need any update")
    }
}

// TODO: improve this function
fn cp_file_to_app_menu(file_name: &OsStr) {
    let mut original_file_path = PathBuf::new();
    original_file_path.push(constants::DESKTOP_PATH);
    original_file_path.push(file_name);

    let mut destination_file_path = PathBuf::new();
    destination_file_path.push(constants::APP_MENU_PATH);
    destination_file_path.push(file_name);

    if destination_file_path.exists() {
        println!("File already exists!");
    } else {
        match fs::copy(original_file_path, destination_file_path) {
            Ok(_) => println!("File copied sucessfull!"),
            Err(_) => println!("Error copying the file!"),
        };
    }
}

fn add_startup_wm_class(steam_id: String, file_path: PathBuf) {
    let mut startup_wm_class = String::new();

    startup_wm_class.push_str(constants::STEAM_STARTUP_WM_CLASS_PATH);
    startup_wm_class.push_str(constants::STEAM_STARTUP_WM_CLASS_PREFIX);
    startup_wm_class.push_str(steam_id.as_str());
    let mut file = OpenOptions::new()
        .append(true) // <- append mode
        .open(file_path)
        .unwrap();

    writeln!(file, "{}", startup_wm_class).unwrap();
    println!("File was updated");
}

fn has_startup_wm_class<'t>(lines: Split<'t, &'t str>) -> bool {
    let mut has_startup_wm_class = false;
    for line in lines {
        has_startup_wm_class = line.contains(constants::STEAM_STARTUP_WM_CLASS_PATH);
    }
    return has_startup_wm_class;
}

// TODO: search more about lifetime parameter
fn find_steam_id<'t>(lines: Split<'t, &'t str>) -> Result<String, String> {
    for line in lines {
        let has_steam_icon = line.contains(constants::STEAM_ICON_PATH);

        if has_steam_icon {
            let steam_id = line.replace(constants::STEAM_ICON_PATH, "").to_owned();
            println!("Found a steam id: {steam_id}");
            return Ok(steam_id);
        }
    }

    println!("The id was not found");
    return Err("The id was not found".to_string());
}
