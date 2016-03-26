use clap::ArgMatches;
use std::env;
use std::process::Command;

use settings::Settings;

pub fn open_shell_at_repo(settings: &Settings, args: &ArgMatches) -> Result<(), (String, i32)> {
    let repository = args.value_of("repository").unwrap();

    let shell = match env::var("SHELL") {
        Ok(val) => val,
        Err(e) => return Err((format!("Could not determine shell to use: {}", e), 1)),
    };

    let repo_path = settings.repo_path(&repository);
    if !repo_path.exists() {
        return Err((format!("Repo path does not exist: {}", repo_path.display()), 1));
    }

    match Command::new(shell).status() {
        Ok(exit_status) => {
            if exit_status.success() {
                Ok(())
            } else {
                let exit_code = exit_status.code().unwrap_or(1);
                Err((format!("Shell failed (exit {})", exit_code), exit_code))
            }
        },
        Err(e) => {
            Err((format!("Unable to execute shell: {}", e), 1))
        }
    }
}
