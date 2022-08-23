
#[derive(Debug)]
struct Gitter {
    path: String,
    fake_file: String
}


impl Gitter {
    pub fn new(path: String) -> Gitter {
        Self {  path: path,
                fake_file: String::from("foo.text")
        }
    }

    pub fn commit(msg: String) {
        
    }
}
