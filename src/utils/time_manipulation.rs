use chrono::{DateTime, ParseError, Utc};

pub fn parse_utc_datetime(s: &str) -> Result<DateTime<Utc>, ParseError> {
    let dt = DateTime::parse_from_rfc3339(s)?;
    Ok(dt.with_timezone(&Utc))
}
