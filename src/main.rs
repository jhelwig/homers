#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

mod cli;
mod cd;
mod settings;

use std::process::exit;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");

    match run_cli() {
        Ok(_) => {},
        Err((message, exit_code)) => {
            error!("{}", message);
            exit(exit_code);
        },
    }
}

fn run_cli() -> Result<(), (String, i32)> {
    let matches = cli::app().get_matches();

    let settings = settings::from_matches(&matches);

    match matches.subcommand_name() {
        Some("cd") => try!(cd::open_shell_at_repo(&settings, matches.subcommand_matches("cd").unwrap())),
        None       => panic!("Subcommand is required, so no idea how we got here!"),
        _          => println!("'{}' is not implemented yet.", matches.subcommand_name().unwrap()),
    };

    Ok(())
}
