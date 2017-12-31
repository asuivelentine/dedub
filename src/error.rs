quick_error!{
    #[derive(Debug)]
    pub enum DedupError {
        FileNotFound(err: ::std::io::Error) { from() }
        HashError(err: ::filehash::error::FilehashError) { from() }
        DirError {}
    } 
}
