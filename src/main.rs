#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

mod cli;

fn main() {
    env_logger::init().expect("Failed to initialize logger.");

    run_cli();
}

fn run_cli<'a>() -> Result<(), &'a str> {
    let matches = cli::app().get_matches();

    let verbose_level = match matches.occurrences_of("verbose") {
        0...4 => matches.occurrences_of("verbose"),
        _ => 4,
    };

    match matches.subcommand_name() {
        Some("cd") => println!("CD!"),
        None       => panic!("Subcommand is required, so no idea how we got here!"),
        _          => println!("'{}' is not implemented yet.", matches.subcommand_name().unwrap()),
    };

    Ok(())
}
