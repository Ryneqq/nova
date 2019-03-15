#![feature(try_from)]
mod args;
mod create;
mod generate;
mod module;

use self::args::*;
// use self::create::*;

use structopt::StructOpt;
use crate::generate::Generate::*;
use crate::{AppArgs, Mode};
// use crate::{create_module, create_submodule};

fn main() {
    let args                = AppArgs::from_args();
    let incorrect_options   = args.structure && args.enumerator;

    match args.module{
        true  if incorrect_options  => eprintln!("You cannot use '-e' and '-s' flags at the same time."),
        false if incorrect_options  => eprintln!("You cannot use '-e' and '-s' flags at the same time."),
        true  if args.structure     => println!("{}", Struct(&args.name).build()),
        true  if args.enumerator    => println!("Enumerator option with module was chosen"),
        false if args.structure     => println!("Structure option without module was chosen"),
        false if args.enumerator    => println!("Enumerator option without module was chosen"),
        true                        => println!("Module option was chosen"),
        _                           => eprintln!("You need to use at least one flag!")
    }
}
