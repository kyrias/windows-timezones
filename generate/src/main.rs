use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    iter,
    path::Path,
};

use anyhow::Context;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span};
use quick_xml::events::Event;
use quote::quote;

const CLDR_VERSION: &'static str = "46-1";

fn main() -> anyhow::Result<()> {
    let filename = format!("windowsZones.{CLDR_VERSION}.xml");
    let path = Path::new(&filename);
    let body = if let Ok(mut file) = File::open(&path) {
        eprintln!("Reading from file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .context("failed to read windows zones from file")?;
        buf
    } else {
        eprintln!("Fetching remote");
        let url = format!("https://raw.githubusercontent.com/unicode-org/cldr/release-{CLDR_VERSION}/common/supplemental/windowsZones.xml");
        let body = reqwest::blocking::get(url)
            .and_then(|r| r.error_for_status())
            .context("failed to get CLDR data file")?
            .text()
            .context("failed to get CLDR data file text content")?;

        let mut f = File::create(&path).context("failed to create windows zones file")?;
        f.write_all(body.as_ref())
            .context("failed to write windows zones file")?;

        body
    };

    let mut reader = quick_xml::reader::Reader::from_str(&body);
    reader.config_mut().check_comments = true;

    let mut buf = Vec::new();
    let mut state = State::default();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("error at position {}: {:?}", reader.buffer_position(), e),
            Ok(event) => {
                if !state.parse_event(event).unwrap() {
                    break;
                }
            }
        };
    }

    println!("{}", generate_enum(state));

    Ok(())
}

// Convert all '/' to '__', all '+' to 'Plus' and '-' to 'Minus', unless
// it's a hyphen, in which case remove it. This is so the names can be used
// as rust identifiers.
//
// Copied from chrono-tz.
// https://github.com/chronotope/chrono-tz/blob/3e4adb2891e1cb9167a32c531cdb2f37eb0aaa30/chrono-tz-build/src/lib.rs#L50-L64
fn convert_bad_chars(name: &str) -> String {
    let name = name.replace("/", "__").replace("+", "Plus");
    if let Some(pos) = name.find('-') {
        if name[pos + 1..]
            .chars()
            .next()
            .map(char::is_numeric)
            .unwrap_or(false)
        {
            name.replace("-", "Minus")
        } else {
            name.replace("-", "")
        }
    } else {
        name
    }
}

#[derive(Debug)]
struct Timezone {
    user_friendly_name: String,
    windows_name: String,
    tzdb_id: String,
    alt_tzdb_ids: HashSet<String>,
}

#[derive(Default, Debug)]
struct State {
    timezones: Vec<Timezone>,
    current_friendly_name: Option<String>,
    current_timezone: Option<Timezone>,
}

impl State {
    fn parse_event(&mut self, event: Event) -> anyhow::Result<bool> {
        match event {
            Event::Eof => {
                if let Some(current) = self.current_timezone.take() {
                    self.timezones.push(current);
                }
                self.current_friendly_name.take();
                return Ok(false);
            }
            Event::Comment(comment) => {
                let timezone = String::from_utf8_lossy(&comment.into_inner())
                    .to_string()
                    .trim()
                    .to_string();
                // Ignore comments unknown comments.  Previously the only comments in the file were
                // the headers dividing up the different time zones, but now there are some
                // additional comments about what Microsoft *may* do in the future.
                if !timezone.starts_with("(UTC") {
                    return Ok(true);
                }

                if let Some(current) = self.current_timezone.take() {
                    self.timezones.push(current);
                }
                self.current_friendly_name = Some(timezone);
            }
            Event::Empty(tag) if tag.local_name().as_ref() == b"mapZone" => {
                let windows_name = String::from_utf8_lossy(
                    &tag.try_get_attribute("other")?
                        .context("missing 'other' attribute")?
                        .value,
                )
                .to_string();

                let territory = String::from_utf8_lossy(
                    &tag.try_get_attribute("territory")?
                        .context("missing 'territory' attribute")?
                        .value,
                )
                .to_string();
                let tzdb_id = String::from_utf8_lossy(
                    &tag.try_get_attribute("type")?
                        .context("missing 'other' attribute")?
                        .value,
                )
                .to_string();

                if territory == "001" {
                    let user_friendly_name = self
                        .current_friendly_name
                        .as_deref()
                        .context("missing time zone header")?
                        .to_string();
                    self.current_timezone = Some(Timezone {
                        user_friendly_name,
                        windows_name,
                        alt_tzdb_ids: iter::once(tzdb_id.clone()).collect(),
                        tzdb_id,
                    });
                } else {
                    if let Some(tz) = &mut self.current_timezone {
                        for fragment in tzdb_id.split(' ').filter(|x| !x.is_empty()) {
                            tz.alt_tzdb_ids.insert(fragment.to_owned());
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(true)
    }
}

fn generate_enum(state: State) -> String {
    let mut type_variants = Vec::new();
    let mut type_jsonschema_variants = Vec::new();
    let mut type_default = Vec::new();

    let mut chrono_tz_variants = Vec::new();

    let mut timezone_windows_names = Vec::new();
    let mut timezone_descriptions = Vec::new();
    let mut tzdb_ids = Vec::new();

    let mut try_from_tzid = Vec::new();
    let mut try_from_windows = Vec::new();

    for timezone in state.timezones {
        let variant_name = convert_bad_chars(&timezone.windows_name).to_upper_camel_case();
        let windows_variant = Ident::new(&variant_name, Span::call_site());
        type_variants.push(windows_variant.clone());
        type_jsonschema_variants.push(variant_name.to_lower_camel_case());
        if timezone.windows_name == "UTC" {
            type_default.push(quote! { #[default] });
        } else {
            type_default.push(quote! {});
        }

        chrono_tz_variants.push(Ident::new(
            &convert_bad_chars(&timezone.tzdb_id),
            Span::call_site(),
        ));

        timezone_windows_names.push(timezone.windows_name.clone());
        timezone_descriptions.push(timezone.user_friendly_name);
        tzdb_ids.push(timezone.tzdb_id);

        let mut alt_ids: Vec<_> = timezone.alt_tzdb_ids.into_iter().collect();
        alt_ids.sort(); // hashset has random order, but generated code be the same every time
        for tzid in alt_ids {
            try_from_tzid.push(Ident::new(&convert_bad_chars(&tzid), Span::call_site()));
            try_from_windows.push(windows_variant.clone());
        }
    }

    let jsonschema_impl = generate_jsonschema_impl(&type_jsonschema_variants);

    let quoted = quote! {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        #[cfg_attr(feature = "strum", derive(strum::EnumIter))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(
            feature = "sqlx",
            derive(sqlx::Type),
            sqlx(type_name = "windows_timezone", rename_all = "snake_case"),
        )]
        pub enum WindowsTimezone {
            #(
                #[doc = #timezone_descriptions]
                #type_default
                #type_variants
            ),*
        }

        impl WindowsTimezone {
            pub fn description(self) -> &'static str {
                match self {
                    #(
                        Self::#type_variants => #timezone_descriptions
                    ),*
                }
            }

            pub fn name(self) -> &'static str {
                match self {
                    #(
                        Self::#type_variants => #timezone_windows_names
                    ),*
                }
            }

            pub fn tzdb_id(self) -> &'static str {
                match self {
                    #(
                        Self::#type_variants => #tzdb_ids
                    ),*
                }
            }
        }

        #[cfg(feature = "chrono-tz-0_9")]
        impl From<WindowsTimezone> for ::chrono_tz_0_9::Tz {
            fn from(value: WindowsTimezone) -> Self {
                match value {
                    #(
                        WindowsTimezone::#type_variants => ::chrono_tz_0_9::Tz::#chrono_tz_variants
                    ),*
                }
            }
        }
        #[cfg(feature = "chrono-tz-0_9")]
        impl TryFrom<::chrono_tz_0_9::Tz> for WindowsTimezone {
            type Error = crate::FromChronoTzError;

            fn try_from(value: ::chrono_tz_0_9::Tz) -> Result<Self, Self::Error> {
                use ::chrono_tz_0_9::Tz;
                match value {
                    #(
                        Tz::#try_from_tzid => Ok(WindowsTimezone::#try_from_windows),
                    )*
                    _ => Err(crate::FromChronoTzError),
                }
            }
        }

        #[cfg(feature = "chrono-tz-0_10")]
        impl From<WindowsTimezone> for ::chrono_tz_0_10::Tz {
            fn from(value: WindowsTimezone) -> Self {
                match value {
                    #(
                        WindowsTimezone::#type_variants => ::chrono_tz_0_10::Tz::#chrono_tz_variants
                    ),*
                }
            }
        }
        #[cfg(feature = "chrono-tz-0_10")]
        impl TryFrom<::chrono_tz_0_10::Tz> for WindowsTimezone {
            type Error = crate::FromChronoTzError;

            fn try_from(value: ::chrono_tz_0_10::Tz) -> Result<Self, Self::Error> {
                use ::chrono_tz_0_10::Tz;
                match value {
                    #(
                        Tz::#try_from_tzid => Ok(WindowsTimezone::#try_from_windows),
                    )*
                    _ => Err(crate::FromChronoTzError),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::str::FromStr for WindowsTimezone {
            type Err = crate::ParseWindowsTimezoneError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let tz = match s {
                    #(
                        #timezone_windows_names => Self::#type_variants,
                    )*
                    _ => return Err(crate::ParseWindowsTimezoneError),
                };

                Ok(tz)
            }
        }

        #jsonschema_impl
    };

    quoted.to_string()
}

fn generate_jsonschema_impl(type_variants_strings: &Vec<String>) -> proc_macro2::TokenStream {
    quote! {
        #[cfg(feature = "schemars")]
        impl schemars::JsonSchema for WindowsTimezone {
            fn schema_name() -> String {
                "WindowsTimezone".to_string()
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                use schemars::schema::{InstanceType, Schema, SchemaObject};
                Schema::Object(SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    enum_values: Some(vec![
                        #(
                            #type_variants_strings.into()
                        ),*
                    ]),
                    ..Default::default()
                })
            }

            fn is_referenceable() -> bool {
                true
            }
        }
    }
}
