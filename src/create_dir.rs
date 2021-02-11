use std::path::Path;
use std::fs;

pub fn create_dir(path: &Path, parents: bool, verbose: bool, _mode: u16) -> i8 {
    if parents {
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("{}: {}", path.display(), e.to_string());
            return 1;
        }
    } else {
        if let Err(e) = fs::create_dir(path) {
            eprintln!("{}: {}", path.display(), e.to_string());
            return 1;
        }
    }

    // Disabled since idk how to change permission with Windows and Linux
    /*if change_permissions(path, fs::Permissions::from_inner()) == 1 {
        return 1;
    }*/

    if verbose {
        println!("mkdir: created directory {}", path.display());
    }

    return 0;
}