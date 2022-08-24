
use lipsum::lipsum;

const LOREM_WORDS: u8 = 8;


#[derive(Debug)]
pub struct Gitter {
    path: String,
    fake_file: String,
    num_commits: u32
}


impl Gitter {
    pub fn new(path: String, file: String) -> Gitter {
        Self {  path: path,
                fake_file: file,
                num_commits: 0
        }
    }
    
    /// Commit
    /// command should look like "git commit --date='year-month-day hour:minutes:seconds' -m "message""
    pub fn commit(&mut self, date_string: String) {
        let msg = lipsum::lipsum_words(LOREM_WORDS.into());    
    }
    
    pub fn init_file() {

    }

    /// Check Repository
    ///
    pub fn check_repository() {

    }

}
