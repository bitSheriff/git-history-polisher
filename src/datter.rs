use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Weekday, Datelike};
use chrono::format::ParseError;

#[derive(Debug)]
pub struct Datter {
    start_date: NaiveDate,
    end_date: NaiveDate,
    only_workday: bool
}

impl Datter {
   

    fn string2date(date_string: String) -> NaiveDate {
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

    pub fn new(start: String, end: String, work: bool) -> Datter {
        Datter { start_date: Datter::string2date(start),
                 end_date: Datter::string2date(end),
                 only_workday: work }
    }

}

