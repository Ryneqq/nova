use std::path::PathBuf;
use structopt::StructOpt;
use std::str::FromStr;
use std::fs::{self, File};

#[derive(StructOpt, Debug)]
pub enum Mode {
    #[structopt(name = "struct")]
    Struct,
    #[structopt(name = "enum")]
    Enum,
    #[structopt(name = "mod")]
    Mod
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "struct"    => Ok(Mode::Struct),
            "enum"      => Ok(Mode::Enum),
            "mod"       => Ok(Mode::Mod),
            _           => Err(String::from("Option not known. Avaible options: [struct|enum|mod]"))
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "nova", about = "1233321")]
pub enum AppArgs {
    #[structopt(name = "add")]
    Add {
        mode: Mode,
        name: String,
        // names: Vec<String>,
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
        // files: Vec<PathBuf>,
    }
}
