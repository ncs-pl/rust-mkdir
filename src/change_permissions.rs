// Disabled since idk how to change permission with Windows and Linux
use std::path::Path;
use std::fs::{set_permissions, Permissions};

pub fn change_permissions(path: &Path, permissions: Permissions) -> i8 {
    if let Err(e) = set_permissions(path, permissions) {
        eprintln!("{}: {}", path.display(), e.to_string());
        return 1;
    }
    return 0;
}