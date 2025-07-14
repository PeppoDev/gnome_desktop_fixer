use std::fs::OpenOptions;
use std::io::Write;
use std::str::Split;
use std::{fs, path::PathBuf};

use crate::constants;

pub fn get_applications_dir() -> PathBuf {
    let dir = dirs::home_dir().expect("Could not get the home directory");
    let dir = dir.join(constants::APP_MENU_PATH);
    return dir;
}

pub fn update_desktop_entry(path: PathBuf) {
    println!("\nReading {}", path.display());

    let file = fs::read_to_string(&path).expect("Should have been able to read the file");
    let content = file.split("\n");

    let has_startup_wm_class = has_startup_wm_class(content.clone());

    let steam_id = match find_steam_id(content.clone()) {
        Ok(id) => id,
        Err(_) => "".to_string(),
    };

    let should_update = !has_startup_wm_class && steam_id != "";

    if should_update {
        add_startup_wm_class(steam_id, &path);
    } else {
        println!("The file do not need any update")
    }
}

fn add_startup_wm_class(steam_id: String, file_path: &PathBuf) {
    let mut startup_wm_class = String::new();

    startup_wm_class.push_str(constants::STEAM_STARTUP_WM_CLASS_PATH);
    startup_wm_class.push_str(constants::STEAM_STARTUP_WM_CLASS_PREFIX);
    startup_wm_class.push_str(steam_id.as_str());
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();

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
