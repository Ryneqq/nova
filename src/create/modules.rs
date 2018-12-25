use std::path::{Path, PathBuf};
use std::fs::{self, File};

const MOD_RS:       &str = "mod.rs";
const LIB_RS:       &str = "lib.rs";
const DEFAULT_PATH: &str = ".";

pub fn create_module(path: Option<PathBuf>, name: &str) {
    let path = path.unwrap_or(DEFAULT_PATH.into());

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

    panic!("Nothing written! Haven't found any 'mod.rs' nor 'lib.rs' under path: '{:?}'", path);
}

fn mod_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(MOD_RS);
    return_path_when_exists(path)
}

fn lib_rs_exists<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref().join(LIB_RS);
    return_path_when_exists(path)
}

fn return_path_when_exists(path: PathBuf) -> Option<PathBuf> {
    match path.exists() {
        true  => Some(path),
        false => None
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

fn create_module_as_folder<P: AsRef<Path>>(path: P, name: &str) {
    let mod_path = path.as_ref().join(name).as_path().join(MOD_RS);
    fs::create_dir_all(mod_path.parent().unwrap()).unwrap();
    let mut file = File::create(&mod_path).unwrap();
    let _ = file.sync_all();

    println!("Everithing gone smoothly. You had luck this time...");
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "tmp/my_mod";

    #[test]
    fn name() {
        create_module(Some(PATH.into()), "my_new_mod");
        // create_submod(Some(PATH.into()), "my_new_struct");
    }
}
