use ansi_term::Colour::{Blue, Green, Red};
use clap::ArgMatches;
use git2::Repository;
use std::env;
use std::fs;
use std::os::unix;
use std::path::{Path, PathBuf};
use std::string::ToString;

use settings::Settings;

pub struct RepoLink {
    source: PathBuf,
    destination: PathBuf,
}

impl RepoLink {
    pub fn new(src: PathBuf, dst: PathBuf) -> RepoLink {
        RepoLink {
            source: src,
            destination: dst,
        }
    }

    pub fn source(&self) -> &PathBuf {
        &self.source
    }

    pub fn destination(&self) -> &PathBuf {
        &self.destination
    }

    pub fn exists(&self) -> bool {
        self.source().exists()
    }

    pub fn correct(&self) -> bool {

        let source_link = match fs::read_link(self.source()) {
            Ok(m) => m,
            Err(e) => {
                error!("Unable to read symlink ({}): {}",
                       self.source().display(),
                       e);
                return false;
            }
        };

        source_link.as_path() == self.destination().as_path()
    }
}

pub fn clone(settings: &Settings, args: &ArgMatches) -> Result<(), (String, i32)> {
    let url = args.value_of("uri").unwrap();
    let repo_name = args.value_of("name").unwrap();
    let repo_path = settings.repo_path(repo_name);

    match Repository::clone(url, repo_path) {
        Ok(_) => Ok(()),
        Err(e) => Err((format!("Failed to clone repository: {}", e), 2)),
    }
}

pub fn link_repo(settings: &Settings, args: &ArgMatches) -> Result<(), (String, i32)> {
    let repo_name = args.value_of("repository").unwrap();
    let repo_links = try!(links_for_repo(&settings, &repo_name));

    for repo_link in repo_links {
        let link_status;

        if repo_link.exists() {
            if repo_link.correct() {
                link_status = Blue.bold().paint("identical");
            } else {
                link_status = Red.bold().paint("conflict");
            }
        } else {
            link_status = Green.bold().paint("symlink");
        }

        // The .to_string() is here, since an ANSIString doesn't know how to pad itself, but a
        // String does.
        //
        // 12 would get us the spacing we want, IF we weren't counting the invisible color control
        // codes, but since we are (due to ANSIString -> String conversion), we need to bump the
        // field out to 22 characters.
        //
        // :sad_trombone:
        println!("{: >22} {}",
                 link_status.to_string(),
                 repo_link.source().display());
    }

    Ok(())
}

fn links_for_repo(settings: &Settings, repo_name: &str) -> Result<Vec<RepoLink>, (String, i32)> {
    let repo_path = settings.repo_path(repo_name);
    let repo_dotfiles_path = repo_path.join("home");
    let dir_entries = match fs::read_dir(&repo_dotfiles_path) {
        Ok(d) => d,
        Err(e) => return Err((format!("Unable to read repository directory ({}): {}",
                                      repo_dotfiles_path.display(),
                                      e),
                              2)),
    };

    let home_env = match env::var("HOME") {
        Ok(h) => h.to_string(),
        _ => return Err((String::from("Unable to find HOME"), 1)),
    };
    let home_path = Path::new(&home_env);

    let mut repo_links = Vec::new();

    for dir_entry in dir_entries {
        let entry = match dir_entry {
            Ok(e) => e,
            Err(e) => return Err((format!("Unable to read directory entry: {}", e), 3)),
        };

        let entry_path = entry.path();
        let repo_relative_path = match entry_path.strip_prefix(&repo_dotfiles_path) {
            Ok(p) => p,
            Err(e) => return Err((format!("Unable to get relative path to repo entry: {}", e),
                                  3)),
        };

        let source_path = home_path.join(repo_relative_path);
        let dest_path = entry_path.to_path_buf();
        trace!("Potential link: {} -> {}",
               source_path.display(),
               dest_path.display());
        repo_links.push(RepoLink::new(source_path, dest_path));
    }

    Ok(repo_links)
}
