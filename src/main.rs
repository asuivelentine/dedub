extern crate clap;
extern crate filehash;
#[macro_use] extern crate quick_error;

use std::path::Path;
use std::process::exit;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;
use std::hash::Hash;

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
        .get_matches();

    //safe unwrap since cargo will require this argument
    let path = matches.value_of("path").unwrap();
    let path = Path::new(path);

    if !path.is_dir() {
        println!("Argument is not a valid path to a directory");
        exit(1);
    }

    hash_files(path);
}

fn yell<S: Debug>(first: Option<S>, second: &S) {
    match first {
        Some(f) => println!("{:?} {:?}", f, second),
        _ => (),
    };
}

fn hash_files(dir: &Path) -> Result<HashMap<Vec<u8>, OsString>> {
    let mut files = HashMap::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let map = hash_files(&path)?;
            files = merge(&files, &map);
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

fn merge<K: Hash + Eq + Clone, V: Clone+ Debug>(first_context: &HashMap<K, V>, second_context: &HashMap<K, V>) -> HashMap<K, V> {
    let mut new_context = HashMap::new();
    for (key, value) in first_context.iter() {
        new_context.insert(key.clone(), value.clone());
    }
    for (key, value) in second_context.iter() {
        let res = new_context.insert(key.clone(), value.clone());
        yell(res, value);
    }
    new_context
}
