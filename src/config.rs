use std::ffi::OsString;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    path: OsString,
    ignore_empty: bool,
    ignore_link: bool,
}

impl Config  {
    pub fn new<S: Into<String>>(dir: S) -> Config {
        Config {
            path: OsString::from(dir.into()),
            ignore_empty: false,
            ignore_link: false,
        }
    }

    pub fn default_config<S: Into<String>>(dir: S) -> Config {
        Config::new(dir.into())
            .with_ignore_link()
            .with_ignore_empty()
    }

    pub fn update_path<S: Into<String>>(self, path: S) -> Config {
        let mut cfg = Config::new(path);
        if self.ignore_link {
            cfg = cfg.with_ignore_link();
        }
        if self.ignore_empty {
            cfg = cfg.with_ignore_empty();
        }
        cfg
    }

    pub fn with_ignore_empty(mut self) -> Config {
        self.ignore_empty = true;
        self
    }

    pub fn with_ignore_link(mut self) -> Config {
        self.ignore_link = true;
        self
    }

    pub fn dir(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn ignore_links(&self) -> bool {
        self.ignore_link
    }

    pub fn ignore_emptys(&self) -> bool {
        self.ignore_empty
    }
}

