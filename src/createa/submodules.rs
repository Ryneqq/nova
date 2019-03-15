use std::path::{Path, PathBuf};
use std::fs::{self, File};
use crate::Mode;

const DEFAULT_PATH:     &str = ".";
const TEMPLATES_PATH:   &str = "templates/struct.rs";
const MOD_RS:           &str = "mod.rs";
const LIB_RS:           &str = "lib.rs";

pub fn create_submodule(path: &Option<PathBuf>, name: &str, mode: &Mode) {
    let path = path.clone().unwrap_or(DEFAULT_PATH.into());

    if let Some(mod_path) = mod_rs_exists(&path) {
        import_module(&mod_path, &name);
        create_module_as_file(&path, &name);
        return
    }

    if let Some(lib_path) = lib_rs_exists(&path) {
        import_module(&lib_path, &name);
        create_module_as_file(&path, &name);
        return
    }
}

fn import_module<P: AsRef<Path>>(path: P, name: &str) {
    let mut content = fs::read_to_string(&path)
        .unwrap()
        .trim()
        .to_string();

    if !content.is_empty() {
        content.push('\n');
    }

    let to_write = format!("mod {};\npub use self::{}::*;\n", name, name);

    if content.contains(&to_write) {
        panic!("Module already imported!")
    } else {
        content.push_str(&to_write);
        fs::write(&path, content).expect("Couldn't save that.");
    }
}

fn create_module_as_file<P: AsRef<Path>>(path: P, name: &str) {
    let mut path = path.as_ref().join(name);
    path.set_extension("rs");
    // let _    = fs::copy(TEMPLATES_PATH, &path);
    fs::copy(TEMPLATES_PATH, &path).expect(&format!("Could not copy this file: '{:?}'", path));



    println!("Everithing gone smoothly. You had luck this time...");
}

fn mod_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(MOD_RS);

    match path.exists() {
        true  => Some(path),
        false => None
    }
}

fn lib_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(LIB_RS);

    match path.exists() {
        true  => Some(path),
        false => None
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const PATH: &str = "tmp/my_mod";

//     #[test]
//     fn name() {
//         // create_mod(Some(PATH.into()), "my_new_mod");
//         create_submod(Some(PATH.into()), "my_new_struct");
//     }
// }
