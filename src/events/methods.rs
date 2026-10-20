use time::{Date, OffsetDateTime};

pub fn parse_date(date_str: &str) -> Result<Date, String> {
    Date::parse(
        date_str,
        time::macros::format_description!("[year]-[month]-[day]"),
    )
    .map_err(|e| format!("Expected dates to have format YYYY-MM-DD: {e}"))
}

pub fn today() -> Date {
    OffsetDateTime::now_utc().date()
}
