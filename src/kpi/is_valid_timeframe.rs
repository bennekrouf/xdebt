
use chrono::NaiveDate;

pub fn is_valid_timeframe(release_date: &Option<NaiveDate>, eol: &Option<NaiveDate>, extended_end_date: &Option<NaiveDate>, today: NaiveDate) -> bool {
    if let Some(start) = release_date {
        if today < *start {
            return false;
        }
    }

    if let Some(end) = eol {
        if today > *end {
            if let Some(extended_end) = extended_end_date {
                return today <= *extended_end;
            } else {
                return false;
            }
        }
    }

    true
}
