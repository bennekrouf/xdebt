
use chrono::NaiveDate;

pub fn is_valid_timeframe(
    release_date: &Option<NaiveDate>,
    eol: &Option<NaiveDate>,
    extended_end_date: &Option<NaiveDate>,
    today: NaiveDate
) -> bool {
    // Ensure the version has been released
    if let Some(release) = release_date {
        if today < *release {
            return false; // Version hasn't been released yet
        }
    }

    // Check EOL
    if let Some(eol_date) = eol {
        if today > *eol_date {
            return false; // Version's end-of-life has passed
        }
    }

    // Check extended EOL
    if let Some(extended_eol_date) = extended_end_date {
        if today > *extended_eol_date {
            return false; // Version's extended end-of-life has passed
        }
    }

    true // Version is within its valid timeframe
}

