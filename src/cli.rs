use clap::Parser;

use crate::mass_update;
use crate::update;
use crate::watch;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Watch for changes on app menu directory
    #[arg(short, long, action)]
    watch: bool,

    /// Mass update app menu directory
    #[arg(short, long)]
    mass_update: bool,

    /// Update binary
    #[arg(short, long)]
    update: bool,
}

pub fn cli_handler() {
    let args = Args::parse();

    println!("CLI was called with: watch->");
    println!(
        "watch->{}, mass_update->{}, update->{}",
        args.watch, args.mass_update, args.update
    );

    if args.update {
        println!("Starting update binary");
        update::run();
    }
    if args.mass_update {
        println!("Starting mass update");
        mass_update::run();
    }
    if args.watch {
        println!("Starting watch mode");
        watch::run()
    }
}
