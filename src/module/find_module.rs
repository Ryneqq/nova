use std::path::{Path, PathBuf};

pub const MOD_RS:   &str = "mod.rs";
pub const LIB_RS:   &str = "lib.rs";
pub const MAIN_RS:  &str = "main.rs";

pub struct ModFinder {
    path:   PathBuf,
}

impl ModFinder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().into();

        Self {
            path
        }
    }
}

fn verify_modules<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    let root = Path::new("./");

    if path == root {
        return verify_module(&path);
    } else
    if ends_with_module(&path) {
        let path = path.parent()
            .unwrap_or(Path::new(""));

        return verify_modules(&path);
    }

    !path.ancestors()
        .take_while(|path| *path != root)
        .map(|path| verify_module(path))
        .any(|mod_exists| !mod_exists)
}

fn verify_module<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    path.join(MOD_RS).exists() ||
    path.join(LIB_RS).exists() ||
    path.join(MAIN_RS).exists()
}

fn ends_with_module<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    match path.exists() {
        true if path.ends_with(MOD_RS)  => true,
        true if path.ends_with(LIB_RS)  => true,
        true if path.ends_with(MAIN_RS) => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};

    #[test]
    fn name() {
        assert_eq!(true, verify_modules("./src/module"));
        assert_eq!(true, verify_modules("./src/module/mod.rs"));

        File::create("./mod.rs");
        assert_eq!(true, verify_modules("./mod.rs"));
        assert_eq!(true, verify_modules("./"));
        fs::remove_file("./mod.rs");

        assert_eq!(false, verify_modules("./"));
        assert_eq!(false, verify_modules("./src/module/123"));
    }
}
