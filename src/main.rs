mod create_dir;
// mod change_permissions; Disabled since idk how to change permission with Windows and Linux

extern crate clap;
use clap::{App, Arg, Values};
use std::path::Path;

const VERSION: &str = "1.0.0";
const OPTION_MODE: &str = "mode";
const OPTION_PARENTS: &str = "parents";
const OPTIONS_VERBOSE: &str = "verbose";
const ARGUMENT_DIRECTORIES: &str = "DIRECTORIES";

fn main() {
    let matches = App::new("mkdir")
        .version(VERSION)
        .about("mkdir - make directories")
        .arg(
            Arg::with_name(OPTION_MODE)
                .short("m")
                .long("mode")
                .help("DISABLED: set file mode (as in chmod), not a=rwx - umask")
                .default_value("755")
        )
        .arg(
            Arg::with_name(OPTION_PARENTS)
                .short("p")
                .long("parents")
                .help("no error if existing, make parent directories as needed")
        )
        .arg(
            Arg::with_name(OPTIONS_VERBOSE)
                .short("v")
                .long("verbose")
                .help("print a message for each created directory")
        )
        .arg(
            Arg::with_name(ARGUMENT_DIRECTORIES)
                .multiple(true)
                .min_values(1)
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let dirs: Vec<String> = matches.values_of(ARGUMENT_DIRECTORIES).map(
        |values: Values| values.map(
            |dir_name: &str| dir_name.to_string()
        ).collect()
    ).unwrap();
    let _mode: &str = matches.value_of(OPTION_MODE).unwrap();
    let parents: bool = matches.is_present(OPTION_PARENTS);
    let verbose: bool = matches.is_present(OPTIONS_VERBOSE);

    for dir in &dirs {
        let path: &Path = Path::new(dir);
        if !parents {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    eprintln!("cannot create directory '{}': No such file of directory", path.display());
                    continue;
                }
            }
        }
        create_dir::create_dir(path, parents, verbose, 0o755);
    }
}
