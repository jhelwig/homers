use clap::ArgMatches;
use git2::Repository;

use settings::Settings;

pub fn clone(settings: &Settings, args: &ArgMatches) -> Result<(), (String, i32)> {
    let url = args.value_of("uri").unwrap();
    let repo_name = args.value_of("name").unwrap();
    let repo_path = settings.repo_path(repo_name);

    match Repository::clone(url, repo_path) {
        Ok(_)  => Ok(()),
        Err(e) => Err((format!("Failed to clone repository: {}", e), 2)),
    }
}
