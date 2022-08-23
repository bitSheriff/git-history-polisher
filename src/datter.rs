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
        self.iterate_date += Duration::days(1)
    }

    pub fn new(start: String, end: String, work: bool) -> Datter {
        Datter { start_date: Datter::string2date(&start),
                 end_date: Datter::string2date(&end),
                 iterate_date: Datter::string2date(&start),
                 only_workday: work,
                 has_finished: false }
    }

    fn iterate_internal_date(&mut self){
        // interate date
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

        let final_string = format!("{} {}:{}:{}", date_string, rand_hour.to_string(), rand_min.to_string(), rand_second.to_string());

        // increment date
        self.iterate_internal_date();

        Ok(final_string)
    }

    pub fn get_finished(&self) -> bool {
        self.has_finished
    }

}

