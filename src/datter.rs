
#[derive(Debug)]
struct Datter {
    start_date: String,
    end_date: String,
    only_workday: bool
}

impl Datter {
    pub fn new(start: String, end: String, work: bool) -> Datter {
        Datter { start_date: start, end_date: end, only_workday: work }
    }
}
