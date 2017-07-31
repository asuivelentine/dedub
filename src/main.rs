extern crate clap;
extern crate filehash;

use clap::{Arg, App};

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
}
