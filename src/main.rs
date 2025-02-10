// src/main.rs

use log::{error, info};
use std::env;

mod rule_manager;
mod file_mover;
mod file_organizer;
mod cli;  // Import cli module

fn main() {
    env_logger::init();

    // Build the CLI and get matches
    let matches = cli::build_cli().get_matches();

    // If no arguments are provided, show a message
    if cli::handle_no_arguments(&matches) {
        return;
    }

    // Handle the commands passed to the application
    cli::handle_commands(&matches);
}
