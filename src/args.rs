use std::path::PathBuf;
use structopt::StructOpt;
use std::str::FromStr;
use std::fs::{self, File};

#[derive(StructOpt, Debug)]
pub enum Mode {
    #[structopt(name = "s")]
    Struct,
    #[structopt(name = "e")]
    Enum,
    #[structopt(name = "m")]
    Mod
}

// nova -s my_struct ./src/dummy/path => creates struct under path
// nova -e my_enum ./src/dummy/path => creates enum under path
// nova -m my_mod ./src/dummy/path => createsempty mod inside a folder
// nova -sm my_struct ./src/dummy/path => creates module and put a new struct with the same name under this path


#[derive(StructOpt, Debug)]
#[structopt(name = "nova", about = "1233321")]
pub struct AppArgs {
    // pub mode: Mode,
    #[structopt(short = "m", long = "mod")]
    pub module:     bool,
    #[structopt(short = "s", long = "struct")]
    pub structure:  bool,
    #[structopt(short = "e", long = "enum")]
    pub enumerator: bool,
    pub name:       String,
    #[structopt(parse(from_os_str))]
    pub path:       Option<PathBuf>,
}
