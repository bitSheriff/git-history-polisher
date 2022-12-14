
use lipsum::lipsum;
use rand::Rng;
use crate::systemer;

const LOREM_WORDS: u8 = 8;


#[derive(Debug)]
pub struct Gitter {
    path: String,
    fake_file: String,
    min_commits: u16,
    max_commits: u16,
    num_commits: u32,
}


impl Gitter {
    pub fn new(path: String, file: String, min: u16, max: u16) -> Gitter {
        Self {  path: path,
                fake_file: file,
                min_commits: min,
                max_commits: max,
                num_commits: 0
        }
    }
    
    pub fn commit_on_date(&mut self, date_string: &String) {
        let mut commits:u16 = 1;

        if self.min_commits != self.max_commits
        {
            let mut rng = rand::thread_rng();
            commits = rng.gen_range(self.min_commits .. self.max_commits);
        }

        for _i in 0..commits {
            self.commit(&date_string);
        }
    }

    /// Commit
    /// command should look like "git commit --date='year-month-day hour:minutes:seconds' -m "message""
    fn commit(&mut self, date_string: &String) {
        let msg = lipsum::lipsum_words(LOREM_WORDS.into());
        let file_with_folder = format!("{}/{}", &self.path, & self.fake_file);
        let _ = systemer::sys_change_file(&file_with_folder);

        systemer::git_add(&self.path);

        systemer::git_commit(&self.path, date_string, msg);

        // increase the the internal commit counter
        self.num_commits += 1;
    }
    
    pub fn get_num_commits(&self) -> u32 {
        self.num_commits
    }

    /// Check Repository
    ///
    pub fn check_repository() {
        todo!()
    }

    pub fn clean_up(&mut self){

    }

}
