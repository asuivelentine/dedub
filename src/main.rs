extern crate clap;
extern crate filehash;
#[macro_use] extern crate quick_error;

use std::path::Path;
use std::process::exit;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;

use clap::{Arg, App};
use filehash::filehash::Filehash;

pub mod config;
pub mod error;

use config::Config;
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
             .short("e")
             .value_name("ignoreempty")
             .help("Ignore empty files since it would have the same hash")
             .takes_value(false))
        .arg(Arg::with_name("ignorelinks")
             .short("l")
             .value_name("ignorelinks")
             .help("Ignore links since it would have the same hash")
             .takes_value(false))
        .get_matches();

    //safe unwrap since cargo will require this argument
    let path = matches.value_of("path").unwrap();
    let mut cfg = Config::new(path);

    if matches.is_present("ignoreempty") {
        cfg = cfg.with_ignore_empty();
    }

    if matches.is_present("ignorelinks") {
        cfg = cfg.with_ignore_link();
    }

    if !cfg.dir().is_dir() {
        println!("Argument is not a valid path to a directory");
        exit(1);
    }

    match hash_files(cfg) {
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

fn check_current_file(cfg: &Config, path: &Path) -> Result<Option<()>> {
    let ignore_empty = cfg.ignore_emptys();
    let ignore_link = cfg.ignore_links();
    let metadata = fs::symlink_metadata(path)?;

    if ignore_link && metadata.file_type().is_symlink() {
        return Ok(None);
    }

    if ignore_empty && (metadata.len() == 0) {
        return Ok(None);
    }

    Ok(Some(()))
}


fn hash_files(cfg: Config) -> Result<HashMap<Vec<u8>, OsString>> {
    let mut files = HashMap::new();
    let _ = hash_files_rec(&cfg, &mut files)?;
    Ok(files)
}

fn hash_files_rec<'a>(cfg: &Config, files: &mut HashMap<Vec<u8>, OsString>)
    -> Result<()> {
    let dir = cfg.dir();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if check_current_file(cfg, &path)?.is_none() {
            continue;
        } 

        if path.is_dir() {
            let cfg = try!(path.to_str()
                .ok_or(DedupError::DirError)
                .map(|s| cfg.clone().update_path(s)));

            hash_files_rec(&cfg, files)?;
            continue;
        } 

        let path = path.into_os_string();
        let hash = Filehash::new(path.clone())
            .hash()?;
        let res = files.insert(hash, path.clone());
        yell(res, &path);
    }
    Ok(())
}

