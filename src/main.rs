extern crate clap;
extern crate filehash;
#[macro_use] extern crate quick_error;

use std::path::Path;
use std::process::exit;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;

use clap::{Arg, App};
use filehash::filehash::Filehash;

pub mod error;

use error::DedupError;

pub type Result<T> = ::std::result::Result<T, DedupError>;

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
        .arg(Arg::with_name("ignoreempty")
             .short("i")
             .value_name("ignoreempty")
             .help("Ignore empty files since it would have the same hash")
             .takes_value(false))
        .get_matches();

    //safe unwrap since cargo will require this argument
    let path = matches.value_of("path").unwrap();
    let path = Path::new(path);

    if matches.is_present("ignoreempty") {
        println!("search ignoring empty files");
    }

    if !path.is_dir() {
        println!("Argument is not a valid path to a directory");
        exit(1);
    }

    match hash_files(path) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };
}

fn yell<S: Debug>(first: Option<S>, second: &S) {
    match first {
        Some(f) => println!("{:?} {:?}", f, second),
        _ => (),
    };
}

fn hash_files(dir: &Path) -> Result<HashMap<Vec<u8>, OsString>> {
    let files = HashMap::new();
    hash_files_rec(dir, files)
}

fn hash_files_rec(dir: &Path, mut files: HashMap<Vec<u8>, OsString>)
    -> Result<HashMap<Vec<u8>, OsString>> {

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files = hash_files_rec(&path, files)?;
            continue;
        } 

        let len = try!(path.metadata())
            .len();
        if len == 0 {
            continue;
        }

        let path = path.into_os_string();
        let hash = Filehash::new(path.clone())
            .hash()?;
        let res = files.insert(hash, path.clone());
        yell(res, &path);
    }
    Ok(files)
}
