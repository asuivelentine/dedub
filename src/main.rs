extern crate clap;
extern crate filehash;

use std::path::Path;
use std::process::exit;
use std::io;

use clap::{Arg, App};
use filehash::filehash::Filehash;

fn main() {
    let matches = App::new("Dedup")
        .version("0.1.0")
        .author("asui <k.sickeler@gmail.com>")
        .about("Find duplicated files")
        .arg(Arg::with_name("path")
             .short("p")
             .long("path")
             .value_name("path")
             .help("Search for dupliates within the given path")
             .required(true)
             .takes_value(true))
        .get_matches();

    //safe unwrap since cargo will require this argument
    let path = matches.value_of("path").unwrap();
    let path = Path::new(path);

    if !path.is_dir() {
        println!("Argument is not a valid path to a directory");
        exit(1);
    }
}
