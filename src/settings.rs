use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    repo_base_dir: PathBuf,
    verbose_level: u8,
}

pub fn from_matches(matches: &ArgMatches) -> Settings {
    let repo_base = match matches.value_of("homers-dir") {
        Some(x) => Path::new(x).to_path_buf(),
        None => {
            Path::new(&env::var("HOME")
                           .unwrap_or_else(|_| panic!("Unable to find HOME"))
                           .to_string())
                .join(".homers")
        }
    };

    let verbose_level = match matches.occurrences_of("verbose") {
        0...4 => matches.occurrences_of("verbose"),
        _ => 4,
    };

    Settings {
        repo_base_dir: repo_base,
        verbose_level: verbose_level as u8,
    }
}

impl Settings {
    pub fn repo_base_dir(&self) -> &PathBuf {
        &self.repo_base_dir
    }

    pub fn repo_path(&self, repository: &str) -> PathBuf {
        self.repo_base_dir().join(repository)
    }

    pub fn verbose_level(&self) -> u8 {
        self.verbose_level
    }
}
