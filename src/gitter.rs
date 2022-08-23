#![crate_name = "gitter"]
#[derive(Debug)]
pub struct Gitter {
    path: String,
    fake_file: String
}


impl Gitter {
    pub fn new(path: String, file: String) -> Gitter {
        Self {  path: path,
                fake_file: file
        }
    }

    pub fn commit(msg: String) {
        
    }
    
    pub fn init_file() {

    }

    /// Check Repository
    ///
    pub fn check_repository() {

    }
}
