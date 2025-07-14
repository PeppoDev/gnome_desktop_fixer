use std::ffi::OsStr;
use std::path::PathBuf;

use inotify::EventMask;
use inotify::{Inotify, WatchMask};

use crate::utils;

fn watch_directory() -> Inotify {
    let inotify = Inotify::init().expect("Failed to initialize `inotify");
    let mut watched_path = PathBuf::new();

    watched_path.push(utils::get_applications_dir());

    inotify
        .watches()
        .add(watched_path, WatchMask::CREATE)
        .expect("Failed to add inotify watch");

    return inotify;
}

fn on_file_creation(raw_file: Option<&OsStr>) {
    let file_name = raw_file.unwrap();
    let mut file_path = PathBuf::new();

    file_path.push(utils::get_applications_dir());
    file_path.push(file_name);

    utils::update_desktop_entry(file_path);
}

pub fn run_watch() {
    let mut inotify = watch_directory();
    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            let is_dir = event.mask.contains(EventMask::ISDIR);

            if event.mask.contains(EventMask::CREATE) {
                if !is_dir {
                    on_file_creation(event.name)
                }
            }
        }
    }
}
