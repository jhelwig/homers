use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    repo_base_dir: PathBuf,
}

pub fn from_matches(matches: &ArgMatches) -> Settings {
    let repo_base = match matches.value_of("homers-dir") {
        Some(x) => Path::new(x).to_path_buf(),
        None    => Path::new(&env::var("HOME").unwrap_or_else(|_| panic!("Unable to find HOME")).to_string()).join(".homers"),
    };

    Settings {
        repo_base_dir: repo_base,
    }
}

impl Settings {
    pub fn repo_base_dir(&self) -> &PathBuf {
        &self.repo_base_dir
    }

    pub fn repo_path(&self, repository: &str) -> PathBuf {
        self.repo_base_dir().join(repository)
    }
}
