mod constants;
mod utils;
mod watch;

fn main() {
    println!("SEARCHING ON APP MENU DIR");
    utils::mass_update(&utils::get_applications_dir());
    println!("\n\nSEARCHING ON DESKTOP DIR");
    utils::mass_update(&utils::get_desktop_dir());

    // this will keep the program running and update whenever the desk dir receives a file
    watch::run_watch();
}
