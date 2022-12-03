use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Weekday, Datelike, Duration};
use chrono::format::ParseError;
use rand::Rng;


#[derive(Debug)]
pub struct Datter {
    start_date: NaiveDate,
    end_date: NaiveDate,
    only_workday: bool,
    iterate_date: NaiveDate,
    has_finished: bool,
    steps: u8,
}

impl Datter {
   

    fn string2date(date_string: &String) -> NaiveDate {
        let date = NaiveDate::parse_from_str(&date_string, "%Y-%m-%d");
        date.unwrap()
    }

    fn is_workday(date: NaiveDate) -> bool {
        let is_work;

        match date.weekday() {
            Weekday::Sat | Weekday::Sun => is_work = false,
            _ => is_work = true,
        }

        is_work
    }

    fn next_date(&mut self) {
        self.iterate_date += Duration::days(self.steps.into());

        // check if only workday is configured
        if self.only_workday == true
        {
            // check if selected day is a workday
            loop
            {
                if Datter::is_workday(self.iterate_date) == false
                {
                    // add only one day
                    self.iterate_date += Duration::days(1);
                }
                else
                {
                    // found a workday
                    break;
                }
            }
        }
    }

    pub fn new(start: String, end: String, work: bool, commits: f32) -> Datter {
        Datter { start_date: Datter::string2date(&start),
                 end_date: Datter::string2date(&end),
                 iterate_date: Datter::string2date(&start),
                 only_workday: work,
                 has_finished: false,
                 steps: Datter::get_steps_from_commits(commits)}
    }

    fn iterate_internal_date(&mut self){
        // iterate date
        self.next_date();

        // check if the end date is reached
        if self.iterate_date > self.end_date
        {
            // no next date, finished
            self.has_finished = true;
        }
    }

    pub fn get_next_date(&mut self) -> Result<String, String> {
        let mut rng = rand::thread_rng();
        let date_string = self.iterate_date.format("%Y-%m-%d").to_string();
        let mut rand_hour : u8 = rng.gen_range(0..24);
        let rand_min : u8 = rng.gen_range(0..59);
        let rand_second : u8 = rng.gen_range(0..59);
        
        if self.has_finished == true
        {
            return Err("has finished".to_string());
        }

        if self.only_workday == true
        {
            // set the commit hours to normal working hours
            rand_hour = rng.gen_range(8..16);
        }

        let final_string = format!("{} {:02}:{:02}:{:02}", date_string, rand_hour, rand_min, rand_second);

        // increment date
        self.iterate_internal_date();

        Ok(final_string)
    }

    pub fn get_finished(&self) -> bool {
        self.has_finished
    }
     

    fn get_steps_from_commits(commits_per_day: f32) -> u8 {
        let mut steps : u8 = 1;
        if commits_per_day < 1.0 
        {
            let steps_f32: f32 = 1.0 / commits_per_day;
            steps = steps_f32.round() as u8;
        }
        steps
    }
}

