use time::{
    OffsetDateTime,
    format_description::well_known::{
        Iso8601,
        iso8601::{Config, TimePrecision},
    },
};

const STAMP: Iso8601<
    {
        Config::DEFAULT
            .set_use_separators(false)
            .set_time_precision(TimePrecision::Second {
                decimal_digits: None,
            })
            .encode()
    },
> = Iso8601;

pub fn timestamp() -> String {
    OffsetDateTime::now_utc().format(&STAMP).unwrap_or_default()
}
