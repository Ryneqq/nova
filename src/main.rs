extern crate nova;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use structopt::StructOpt;
use nova::{AppArgs, Mode};
use nova::create_module as new_create_module;

fn main() {
    let args = AppArgs::from_args();

    match args {
        AppArgs::Add { mode, name, path } =>
            match mode {
                Mode::Mod   => new_create_module(path, &name),
                _           => create_submod(path, &name)
            }
    }
}

const MOD_RS:   &str = "mod.rs";
const LIB_RS:   &str = "lib.rs";

fn create_mod(path: Option<PathBuf>, name: &str) {
    let path = get_path(path);

    if let Some(mod_path) = mod_rs_exists(&path) {
        import_module(&mod_path, &name);
        create_module_as_folder(&path, &name);
        return
    }

    if let Some(lib_path) = lib_rs_exists(&path) {
        import_module(&lib_path, &name);
        create_module_as_folder(&path, &name);
        return
    }
}

fn create_submod(path: Option<PathBuf>, name: &str) {
    let path = get_path(path);

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
    let mut content = fs::read_to_string(&path).unwrap();
    let to_write    = format!("\nmod {};\npub use self::{}::*\n", name, name);

    content.push_str(&to_write);
    fs::write(&path, content).expect("Couldn't save that.");
}

fn create_module_as_folder<P: AsRef<Path>>(path: P, name: &str) {
    create_module(path.as_ref().join(name).as_path().join(MOD_RS))
}

fn create_module_as_file<P: AsRef<Path>>(path: P, name: &str) {
    create_module(path.as_ref().join(name).as_path().join(".rs"));
}

fn create_module<P: AsRef<Path>>(mod_path: P) {
    // match mod_rs_does_not_exist(&mod_path) {
    //     Some(path)  => fs::create_dir_all(path).expect("Something bad happend. I couldn't create your path. Sorryyyy!"),
    //     None        => panic!("Check twice before asking... You already have that module!"),
    // }

    fs::create_dir_all(mod_path.as_ref().parent().unwrap()).unwrap();
    let mut file = File::create(&mod_path).unwrap();
    // file.write_fmt(format_args!("{}", contract_om))?;
    let _ = file.sync_all();

    println!("Everithing gone smoothly. You had luck this time...");
}

const DEFALUT_PATH: &str = ".";

fn get_path(path: Option<PathBuf>) -> PathBuf {
    match path {
        Some(path)  => path,
        None        => DEFALUT_PATH.into()
    }
}

fn mod_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(MOD_RS);

    match path.exists() {
        false => Some(path),
        true  => None
    }
}

fn lib_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(LIB_RS);

    match path.exists() {
        false => Some(path),
        true  => None
    }
}

fn mod_rs_does_not_exist<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(MOD_RS);

    match path.exists() {
        false => Some(path),
        true  => None
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
