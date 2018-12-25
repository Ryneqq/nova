extern crate nova;

use structopt::StructOpt;
use nova::{AppArgs, Mode};
use nova::{create_module, create_submodule};

fn main() {
    let args = AppArgs::from_args();

    match args {
        AppArgs::Add { mode, name, path } =>
            match mode {
                Mode::Mod   => create_module(path, &name),
                _           => create_submodule(path, &name, mode)
            }
    }
}
