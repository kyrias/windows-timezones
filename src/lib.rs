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

#[cfg(all(
    feature = "std",
    any(feature = "chrono-tz-0_9", feature = "chrono-tz-0_10")
))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An error returned when failing to convert a `chrono_tz::Tz` to a [`WindowsTimezone`] using [`TryFrom::try_from`].
pub struct FromChronoTzError;

#[cfg(all(
    feature = "std",
    any(feature = "chrono-tz-0_9", feature = "chrono-tz-0_10")
))]
impl std::fmt::Display for FromChronoTzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "there is no corresponding Windows timezone identifier for the given IANA timezone".fmt(f)
    }
}

#[cfg(all(
    feature = "std",
    any(feature = "chrono-tz-0_9", feature = "chrono-tz-0_10")
))]
impl std::error::Error for FromChronoTzError {}

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

    #[cfg(feature = "chrono-tz-0_9")]
    #[test]
    fn chrono_tz_0_9_conversion() {
        assert_eq!(
            Into::<chrono_tz_0_9::Tz>::into(WindowsTimezone::WEuropeStandardTime),
            chrono_tz_0_9::Tz::Europe__Berlin
        );
        assert_eq!(
            WindowsTimezone::try_from(chrono_tz_0_9::Tz::Europe__Berlin),
            Ok(WindowsTimezone::WEuropeStandardTime)
        );
    }

    #[cfg(feature = "chrono-tz-0_10")]
    #[test]
    fn chrono_tz_0_10_conversion() {
        assert_eq!(
            Into::<chrono_tz_0_10::Tz>::into(WindowsTimezone::WEuropeStandardTime),
            chrono_tz_0_10::Tz::Europe__Berlin
        );
        assert_eq!(
            WindowsTimezone::try_from(chrono_tz_0_10::Tz::Europe__Berlin),
            Ok(WindowsTimezone::WEuropeStandardTime)
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
