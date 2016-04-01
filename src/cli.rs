use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("homers")
        .version(crate_version!())
        .author("Jacob Helwig <jacob@technosorcery.net>")
        .about("Manage dotfiles in your home directory.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("quiet")
                 .short("q")
                 .long("quiet")
                 .help("Suppress output"))
        .arg(Arg::with_name("verbose")
                 .short("v")
                 .long("verbose")
                 .multiple(true)
                 .overrides_with("quiet")
                 .help("Sets the level of verbosity"))
        .arg(Arg::with_name("pretend")
                 .short("p")
                 .long("pretend")
                 .help("Show what would be done, instead of making changes"))
        .arg(Arg::with_name("homers-dir")
                 .short("d")
                 .long("homers-dir")
                 .help("Path to where the repositories are stored (Default: ~/.homers)")
                 .takes_value(true)
                 .value_name("dir")
                 .validator(|val: String| -> Result<(), String> {
                     if Path::new(&val).exists() {
                         Ok(())
                     } else {
                         Err(format!("Path does not exist for --homers-dir: {}", val))
                     }
                 }))
        .subcommand(command_cd())
        .subcommand(command_clone())
        .subcommand(command_commit())
        .subcommand(command_destroy())
        .subcommand(command_diff())
        .subcommand(command_edit())
        .subcommand(command_exec())
        .subcommand(command_exec_all())
        .subcommand(command_generate())
        .subcommand(command_link())
        .subcommand(command_list())
        .subcommand(command_path())
        .subcommand(command_pull())
        .subcommand(command_push())
        .subcommand(command_rc())
        .subcommand(command_status())
        .subcommand(command_track())
        .subcommand(command_unlink())
}

fn command_cd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("cd")
        .about("Open a shell at the root of the given repository")
        .arg(Arg::with_name("repository")
                 .help("The repository to open a shell in")
                 .index(1)
                 .required(true))
}

fn command_clone<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("clone")
        .about("Clone <uri>")
        .arg(Arg::with_name("uri")
                 .help("URI of the repository to clone")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("name")
                 .help("The name to use for the repository locally")
                 .index(2)
                 .required(true))
}

fn command_commit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("commit")
        .about("Comment changes in the given repository")
        .arg(Arg::with_name("repository")
                 .help("Name of the repository in which to commit changes")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("message")
                 .help("The commit message to use")
                 .index(2)
                 .required(true))
}

fn command_destroy<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("destroy")
        .about("Delete all symlinks and remove the repository")
        .arg(Arg::with_name("repository")
                 .help("Name of the repository to destroy")
                 .index(1)
                 .required(true))
}

fn command_diff<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("diff")
        .about("Show the \"git diff\" of uncommitted changes in the repository")
        .arg(Arg::with_name("repository")
                 .help("Name of the repository to \"git diff\"")
                 .index(1)
                 .required(true))
}

fn command_edit<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("edit")
        .about("Open $EDITOR in the root of the given repository")
        .arg(Arg::with_name("repository")
                 .help("The repository to open in $EDITOR")
                 .index(1)
                 .required(true))
}

fn command_exec<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("exec")
        .about("Execute the given command in the root of the specified repository")
        .arg(Arg::with_name("repository")
                 .help("Name of the repository in which to execute the command")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("command")
                 .help("The command to execute")
                 .index(2)
                 .required(true))
}

fn command_exec_all<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("exec_all")
        .about("Execute the given command in the root of the specified repository")
        .arg(Arg::with_name("command")
                 .help("The command to execute")
                 .index(1)
                 .required(true))
}

fn command_generate<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate")
        .about("Create a homers compatible git repository at the specified path")
        .arg(Arg::with_name("path")
                 .help("The path to initialize with a new repository")
                 .index(1)
                 .required(true))
}

fn command_link<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("link")
    //Runtime options:
    //-f, [--force]                    # Overwrite files that already exist
    //-s, [--skip], [--no-skip]        # Skip files that already exist
    .about("Symlink all dotfiles from the specified repository")
    .arg(Arg::with_name("repository")
         .help("The repository to create the symlinks for")
         .index(1)
         .required(true))
}

fn command_list<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List all cloned repositories")
}

fn command_path<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("path")
        .about("Print the path to the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to show the path to")
                 .index(1)
                 .required(true))
}

fn command_pull<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("pull")
        .about("Run \"git pull\" in the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to run \"git pull\" in")
                 .index(1)
                 .required(true))
}

fn command_push<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("push")
        .about("Run \"git push\" in the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to run \"git push\" in")
                 .index(1)
                 .required(true))
}

fn command_rc<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("rc")
        .about("Run the .homersrc from the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to run the .homersrc from")
                 .index(1)
                 .required(true))
}

fn command_status<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("status")
        .about("Run \"git status\" in the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to run \"git status\" in")
                 .index(1)
                 .required(true))
}

fn command_track<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("track")
        .about("Add a file to the specified repository")
        .arg(Arg::with_name("path")
                 .help("The path to add to the repository and make a symlink")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to add the path to")
                 .index(2)
                 .required(true))
}

fn command_unlink<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("unlink")
        .about("Remove all symlinks to the specified repository")
        .arg(Arg::with_name("repository")
                 .help("The name of the repository to remove the symlinks for")
                 .index(1)
                 .required(true))
}
