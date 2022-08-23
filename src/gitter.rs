
#[derive(Debug)]
struct Gitter {
    path: String,
    fake_file: String
}


impl Gitter {
    pub fn new(path: String) -> Gitter {
        Self {  path: path,
                fake_file:"foo.txt".to_string()
        }
    }

    pub fn commit(msg: String) {
        
    }
}
