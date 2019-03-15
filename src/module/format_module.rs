use std::path::{Path, PathBuf};
use std::fs;

const MOD_RS:   &str = "mod.rs";
const LIB_RS:   &str = "lib.rs";
const MAIN_RS:  &str = "main.rs";

pub struct ModFromatter {
    path: PathBuf,
    file: String
}

impl ModFromatter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().into();
        Self {
            path, file: String::new()
        }
    }

    pub fn format(mut self) -> Self {
        let mut file = fs::read_to_string(&self.path).unwrap();
        file.push_str("\n");
        file.push_str("mod xxx;");
        file.push_str("pub use self::snake_case::CamelCase;");
        self.file = file;

        self
    }

    pub fn commit(self) {

    }

    fn load_file(path: &Path) -> String {
        let mut file = String::new();

        String::new()
    }

    fn verify_file(&self) -> Result<(),()> {
        Ok(())
    }
}




// 1..5 same outcome
// 1. nova -s foo_struct ./sample_path/foo_struct/mod.rs (-sm)
// 3. nova -s foo_struct ./sample_path/foo_struct/(does_not_exists)mod.rs (-sm)
// 2. nova -s foo_struct ./sample_path/(does_not_exists)foo_struct/(does_not_exists)mod.rs (-sm)
// 4. nova -sm foo_struct ./sample_path/
// it should create folder 'foo_struct' under 'sample_path'
// then inside folder if they do not exist create 'mod.rs' and 'my_struct.rs'

// 4. nova -s foo_struct sample_path/does_not_exist/does_not_exists/mod.rs //err
