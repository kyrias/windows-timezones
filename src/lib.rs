#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod generated;

pub use generated::WindowsTimezone;

#[cfg(feature = "std")]
impl std::fmt::Display for generated::WindowsTimezone {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.name().fmt(f)
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An error returned when parsing a [`WindowsTimezone`] using [`FromStr::from_str`](std::str::FromStr::from_str) fails.
pub struct ParseWindowsTimezoneError;

#[cfg(feature = "std")]
impl std::fmt::Display for ParseWindowsTimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "the given string was not a known Windows timezone identifier".fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseWindowsTimezoneError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tz = WindowsTimezone::WEuropeStandardTime;

        let description = tz.description();
        assert!(description.starts_with("(UTC+01:00)"));
        assert!(description.contains("Berlin"));
        assert!(description.contains("Stockholm"));

        assert_eq!(tz.tzdb_id(), "Europe/Berlin");
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_str() {
        assert_eq!(
            "W. Europe Standard Time".parse::<WindowsTimezone>(),
            Ok(WindowsTimezone::WEuropeStandardTime),
        );

        assert_eq!(
            WindowsTimezone::WEuropeStandardTime.name(),
            "W. Europe Standard Time",
        );

        assert_eq!(
            "I am a teapot".parse::<WindowsTimezone>(),
            Err(ParseWindowsTimezoneError),
        );
    }

    #[cfg(feature = "chrono-tz")]
    #[test]
    fn chrono_tz_conversion() {
        assert_eq!(
            Into::<chrono_tz::Tz>::into(WindowsTimezone::WEuropeStandardTime),
            chrono_tz::Tz::Europe__Berlin
        );
    }

    #[cfg(feature = "strum")]
    #[test]
    fn strum() {
        use strum::IntoEnumIterator;

        let tzs = WindowsTimezone::iter().collect::<Vec<_>>();
        assert!(tzs.contains(&WindowsTimezone::WEuropeStandardTime));
    }
}
