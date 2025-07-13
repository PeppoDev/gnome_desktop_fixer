mod constants;
mod utils;
mod watch;

fn main() {
    println!("SEARCHING ON APP MENU DIR");
    utils::mass_update(constants::APP_MENU_PATH);
    println!("\n\nSEARCHING ON DESKTOP DIR");
    utils::mass_update(constants::DESKTOP_PATH);

    // this will keep the program running and update whenever the desk dir receives a file
    watch::run_watch();
}
