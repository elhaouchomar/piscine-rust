use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let start = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;
    let is_leap = (year % 4 == 0) && ((year % 100 != 0)|| (year % 400 == 0));

    if is_leap {
        return None;
    }

    let middle = start.checked_add_signed(chrono::Duration::days(182))?;
    Some(middle.weekday())
}
