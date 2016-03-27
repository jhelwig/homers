use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    repo_base_dir: PathBuf,
    verbose_level: i8,
}

impl Settings {
    pub fn repo_base_dir(&self) -> &PathBuf {
        &self.repo_base_dir
    }

    pub fn repo_path(&self, repository: &str) -> PathBuf {
        self.repo_base_dir().join(repository)
    }

    pub fn verbose_level(&self) -> i8 {
        self.verbose_level
    }
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

    let mut verbose_level: i8 = match matches.occurrences_of("verbose") {
        0...4 => matches.occurrences_of("verbose") as i8,
        _ => 4,
    };

    if matches.is_present("quiet") {
        verbose_level = -1;
    }

    Settings {
        repo_base_dir: repo_base,
        verbose_level: verbose_level,
    }
}

