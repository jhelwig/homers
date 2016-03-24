use clap::ArgMatches;

pub fn open_shell_at_repo(args: &ArgMatches) -> Result<(), (String, i32)> {
    println!("CD to: {}", args.value_of("repository").unwrap());

    Ok(())
}
