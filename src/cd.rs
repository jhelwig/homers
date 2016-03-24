use clap::ArgMatches;
use std::env;
use std::process::Command;

pub fn open_shell_at_repo(args: &ArgMatches) -> Result<(), (String, i32)> {
    info!("CD to: {}", args.value_of("repository").unwrap());

    let shell = match env::var("SHELL") {
        Ok(val) => val,
        Err(e) => return Err((format!("Could not determine shell to use: {}", e), 1)),
    };

    info!("Using shell: {}", shell);
    match Command::new(shell).status() {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                Err(("Shell failed".to_string(), exit_status.code().unwrap_or(1)))
            }
        },
        Err(e) => {
            Err((format!("Unable to execute shell: {}", e), 1))
        }
    }
}
