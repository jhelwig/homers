use clap::ArgMatches;
use std::process::Command;

pub fn open_shell_at_repo(args: &ArgMatches) -> Result<(), (String, i32)> {
    println!("CD to: {}", args.value_of("repository").unwrap());

    match Command::new("zsh").status() {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                Err(("Shell failed".to_string(), exit_status.code().unwrap_or(1)))
            }
        },
        Err(e) => {
            Err((format!("{}", e), 1))
        }
    }
}
