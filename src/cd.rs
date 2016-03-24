use clap::ArgMatches;

pub fn open_shell_at_repo<'a>(args: &ArgMatches) -> Result<(), (&'a str, i32)> {
    println!("CD to: {}", args.value_of("repository").unwrap());

    Ok(())
}
