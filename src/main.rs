#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate git2;

mod cd;
mod cli;
mod repository;
mod settings;

use std::process::exit;

use settings::Settings;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");

    match run_cli() {
        Ok(_) => {}
        Err((message, exit_code)) => {
            error!("{}", message);
            exit(exit_code);
        }
    }
}

fn run_cli() -> Result<(), (String, i32)> {
    let matches = cli::app().get_matches();

    let settings = Settings::from_matches(&matches);

    match matches.subcommand() {
        ("cd", Some(m)) => try!(cd::open_shell_at_repo(&settings, m)),
        ("clone", Some(m)) => try!(repository::clone(&settings, m)),
        ("link", Some(m)) => try!(repository::link_repo(&settings, m)),
        (name, _) => println!("'{}' is not implemented yet.", name),
    };

    Ok(())
}
