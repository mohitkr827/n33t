use log::error;
use std::env;

mod rule_manager;
mod file_mover;
mod file_organizer;

fn main() {
    env_logger::init();

    // Get the directory from command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a directory argument is provided
    if args.len() < 2 {
        error!("No file directory specified.");
        return;
    }

    // Use the first argument as the directory
    let dir = &args[1];

    let organizer = file_organizer::FileOrganizer::new(dir);
    if let Err(e) = organizer.organize_files() {
        error!("Error organizing files: {}", e);
    }
}
