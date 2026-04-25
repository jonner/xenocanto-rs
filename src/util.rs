use serde::Deserialize;
use std::{fmt::Display, str::FromStr};

pub fn yes_no_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["yes", "no"])),
    }
}

pub fn maybe_yes_no_to_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "yes" => Ok(Some(true)),
        "no" => Ok(Some(false)),
        "unknown" => Ok(None),
        _ => Err(serde::de::Error::unknown_variant(
            &s,
            &["yes", "no", "unknown"],
        )),
    }
}

struct WrappedF64(f64);

impl<'de> serde::Deserialize<'de> for WrappedF64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_number_from_string(deserializer).map(WrappedF64)
    }
}

pub fn maybe_deserialize_number_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<WrappedF64>::deserialize(deserializer).map(|opt| opt.map(|w| w.0))
}

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    struct StringOrNumber<T>(std::marker::PhantomData<T>);

    impl<'de, T> serde::de::Visitor<'de> for StringOrNumber<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        type Value = T;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string or number")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<T, E> {
            v.parse::<T>().map_err(serde::de::Error::custom)
        }

        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<T, E> {
            self.visit_str(&v)
        }

        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<T, E> {
            v.to_string().parse::<T>().map_err(serde::de::Error::custom)
        }

        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<T, E> {
            v.to_string().parse::<T>().map_err(serde::de::Error::custom)
        }

        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<T, E> {
            v.to_string().parse::<T>().map_err(serde::de::Error::custom)
        }
    }

    deserializer.deserialize_any(StringOrNumber(std::marker::PhantomData))
}

pub fn permissive_time<'de, D>(deserializer: D) -> Result<Option<jiff::civil::Time>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;

    // Attempt standard parse first
    if let Ok(t) = s.parse::<jiff::civil::Time>() {
        return Ok(Some(t));
    }

    // 1. Handle single-digit hours (e.g., "9:30" -> "09:30")
    // We check if the first colon is at index 1
    if let Some(colon_idx) = s.find(':')
        && colon_idx == 1
    {
        s.insert(0, '0');
    }

    // Handle "HH:MM" by appending ":00"
    if s.len() == 5 && s.contains(':') {
        let extended = format!("{}:00", s);
        return extended
            .parse::<jiff::civil::Time>()
            .map_err(serde::de::Error::custom)
            .map(Some);
    }

    tracing::warn!(s, "invalid time format; expected HH:MM or HH:MM:SS",);
    Ok(None)
}

pub fn permissive_date<'de, D>(deserializer: D) -> Result<Option<jiff::civil::Date>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Attempt standard parse first
    if let Ok(t) = s.parse::<jiff::civil::Date>() {
        return Ok(Some(t));
    }

    tracing::warn!(s, "invalid date format",);
    Ok(None)
}

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<jiff::Span, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split(':').collect();

    match parts.as_slice() {
        [m, s] => {
            let m = m.parse::<i64>().map_err(serde::de::Error::custom)?;
            let s = s.parse::<i64>().map_err(serde::de::Error::custom)?;
            Ok(jiff::Span::new().minutes(m).seconds(s))
        }
        [h, m, s] => {
            let h = h.parse::<i64>().map_err(serde::de::Error::custom)?;
            let m = m.parse::<i64>().map_err(serde::de::Error::custom)?;
            let s = s.parse::<i64>().map_err(serde::de::Error::custom)?;
            Ok(jiff::Span::new().hours(h).minutes(m).seconds(s))
        }
        _ => Err(serde::de::Error::custom(
            "Format must be [h:]m:ss or [h:]mm:ss",
        )),
    }
}

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

pub fn deserialize_csv_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    // First, deserialize the entire field as one big string
    let s = String::deserialize(deserializer)?;

    if s.trim().is_empty() {
        return Ok(Vec::new());
    }
    // Split by comma, parse each piece, and collect into a Vec
    s.split(',')
        .map(|item| item.trim().parse::<T>().map_err(serde::de::Error::custom))
        .collect()
}
