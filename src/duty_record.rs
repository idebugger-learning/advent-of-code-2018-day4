use chrono::{NaiveDate, NaiveDateTime, Timelike};

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
pub struct DutyRecord {
    pub datetime: NaiveDateTime,
    pub event: Event,
}

impl DutyRecord {
    pub fn extract_date(&self) -> NaiveDate {
        // Events at 11 PM (23:XX) we have to count as the next day
        if self.datetime.hour() == 23 {
            self.datetime.date().succ()
        } else {
            self.datetime.date()
        }
    }
}
