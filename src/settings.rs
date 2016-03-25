use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    pub repo_base_dir: PathBuf,
}

pub fn from_matches(matches: &ArgMatches) -> Settings {
    Settings {
        repo_base_dir: Path::new(&env::var("HOME").unwrap_or_else(|_| panic!("Unable to find HOME")).to_string()).join(".homers"),
    }
}
