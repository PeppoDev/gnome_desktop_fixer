use inotify::EventMask;

mod constants;
mod utils;

fn main() {
    let mut inotify = utils::watch_directory();

    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            let is_dir = event.mask.contains(EventMask::ISDIR);

            if event.mask.contains(EventMask::CREATE) {
                if !is_dir {
                    utils::on_file_creation(event.name)
                }
            }
        }
    }
}
