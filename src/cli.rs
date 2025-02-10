// src/cli.rs

use clap::{Command, Arg};
use std::fs;

pub fn build_cli() -> Command {
    Command::new("n33t")
        .version("1.0")
        .author("mohitkr827")
        .about("A file organizer CLI tool")
        .subcommand(
            Command::new("run")
                .about("Organize files in the specified directory")
                .arg(
                    Arg::new("directory")
                        .help("Directory to organize")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Show additional information about the tool"),
        )
}

pub fn handle_no_arguments(matches: &clap::ArgMatches) -> bool {
    // Check if there are no subcommands passed
    if matches.subcommand_name().is_none() {
        println!("No arguments provided. Try 'n33t --help' to list all available commands.");
        return true;
    }
    false
}

pub fn show_info() {
    let info = fs::read_to_string("assets/info.txt")
        .unwrap_or_else(|_| "Failed to read the info file.".to_string());
    println!("{}", info);
}

pub fn handle_commands(matches: &clap::ArgMatches) {
    // Handle --info flag
    if let Some(_) = matches.subcommand_matches("info") {
        show_info();
    }

    // Handle --run <directory> subcommand
    if let Some(run_matches) = matches.subcommand_matches("run") {
        if let Some(dir) = run_matches.get_one::<String>("directory") {
            println!("Organizing files in directory: {}", dir);
            // Add your file organizing logic here
        }
    }
}
