use std::fmt::Display;

use crate::{License, LifeStage, Quality, Sex, SoundType, SpeciesGroup, WorldArea};

#[derive(Debug)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    LessThan,
    GreaterThan,
    Matches,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::Matches => write!(f, "="),
        }
    }
}

#[derive(Debug)]
pub enum SearchTerm {
    Group(SpeciesGroup),
    Genus(String),
    Species(String),
    Subspecies(String),
    Recordist(String),
    Country(String),
    Location(String),
    Remarks(String),
    Seen(bool),
    Playback(bool),
    Latitude(Option<Operator>, f64),
    Longitude(Option<Operator>, f64),
    GeoBox(Point, Point),
    SoundType(SoundType),
    OtherType(String),
    Sex(Sex),
    LifeStage(LifeStage),
    RecordingMethod(String),
    RecordingId(u64),
    License(License),
    Quality(Option<Operator>, Quality),
    RecordingDuration(Option<Operator>, f64),
    RecordingDurationRange(f64, f64),
    Area(WorldArea),
    Since(jiff::civil::Date),
    RecordingYear(Option<Operator>, u16),
    RecordingMonth(Option<Operator>, u8),
    Temperature(Option<Operator>, f64),
    Automated(bool),
    Device(String),
    Microphone(String),
    SampleRate(Option<Operator>, u64),
}

fn escape_with_operator(s: &str, op: Option<&Operator>) -> String {
    let needs_quotes = s.find(" ").is_some() || op.is_some();
    let arg = s.replace('"', "\\\"");
    if needs_quotes {
        let op = op.map(|op| op.to_string()).unwrap_or_default();
        format!("\"{op}{arg}\"")
    } else {
        arg
    }
}

fn escape(s: &str) -> String {
    escape_with_operator(s, None)
}

fn yesno(val: bool) -> &'static str {
    match val {
        true => "yes",
        false => "no",
    }
}

impl Display for SearchTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchTerm::Group(species_group) => {
                write!(
                    f,
                    "grp:{}",
                    escape(serde_variant::to_variant_name(&species_group).unwrap())
                )
            }
            SearchTerm::Genus(val) => write!(f, "gen:{}", escape(val)),
            SearchTerm::Species(val) => write!(f, "sp:{}", escape(val)),
            SearchTerm::Subspecies(val) => write!(f, "ssp:{}", escape(val)),
            SearchTerm::Recordist(val) => write!(f, "rec:{}", escape(val)),
            SearchTerm::Country(val) => write!(f, "cnt:{}", escape(val)),
            SearchTerm::Location(val) => write!(f, "loc:{}", escape(val)),
            SearchTerm::Remarks(val) => write!(f, "rmk:{}", escape(val)),
            SearchTerm::Seen(val) => write!(f, "seen:{}", yesno(*val)),
            SearchTerm::Playback(val) => write!(f, "playback:{}", yesno(*val)),
            SearchTerm::Latitude(operator, val) => {
                write!(
                    f,
                    "lat:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchTerm::Longitude(operator, val) => {
                write!(
                    f,
                    "lon:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchTerm::GeoBox(p1, p2) => write!(
                f,
                "box:{},{},{},{}",
                p1.latitude, p1.longitude, p2.latitude, p2.longitude
            ),
            SearchTerm::SoundType(sound_type) => {
                write!(
                    f,
                    "type:{}",
                    escape(serde_variant::to_variant_name(sound_type).unwrap())
                )
            }
            SearchTerm::OtherType(val) => write!(f, "othertype:{}", escape(val)),
            SearchTerm::Sex(sex) => write!(
                f,
                "sex:{}",
                escape(serde_variant::to_variant_name(sex).unwrap())
            ),
            SearchTerm::LifeStage(life_stage) => write!(
                f,
                "stage:{}",
                escape(serde_variant::to_variant_name(life_stage).unwrap())
            ),
            SearchTerm::RecordingMethod(val) => write!(f, "method:{}", escape(val)),
            SearchTerm::RecordingId(id) => write!(f, "nr:{id}"),
            SearchTerm::License(license) => write!(
                f,
                "lic:{}",
                escape(serde_variant::to_variant_name(license).unwrap())
            ),
            SearchTerm::Quality(operator, quality) => {
                write!(
                    f,
                    "q:{}",
                    escape_with_operator(
                        serde_variant::to_variant_name(quality).unwrap(),
                        operator.as_ref()
                    )
                )
            }
            SearchTerm::RecordingDuration(operator, duration) => write!(
                f,
                "len:{}",
                escape_with_operator(&duration.to_string(), operator.as_ref())
            ),
            SearchTerm::RecordingDurationRange(from, to) => write!(f, "len:{from}-{to}"),
            SearchTerm::Area(world_area) => {
                write!(
                    f,
                    "area:{}",
                    escape(serde_variant::to_variant_name(world_area).unwrap())
                )
            }
            SearchTerm::Since(date) => write!(f, "since:{}", date.strftime("%F")),
            SearchTerm::RecordingYear(operator, year) => {
                write!(
                    f,
                    "year:{}",
                    escape_with_operator(&year.to_string(), operator.as_ref())
                )
            }
            SearchTerm::RecordingMonth(operator, month) => {
                write!(
                    f,
                    "month:{}",
                    escape_with_operator(&month.to_string(), operator.as_ref())
                )
            }
            SearchTerm::Temperature(operator, temp) => write!(
                f,
                "temp:{}",
                escape_with_operator(&temp.to_string(), operator.as_ref())
            ),
            SearchTerm::Automated(val) => write!(f, "auto:{}", yesno(*val)),
            SearchTerm::Device(val) => write!(f, "dvc:{}", escape(val)),
            SearchTerm::Microphone(val) => write!(f, "mic:{}", escape(val)),
            SearchTerm::SampleRate(operator, val) => write!(
                f,
                "smp:{}",
                escape_with_operator(&val.to_string(), operator.as_ref())
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tags() {
        assert_eq!(
            "grp:\"land mammals\"",
            SearchTerm::Group(SpeciesGroup::LandMammals).to_string()
        );
        assert_eq!(
            "grp:birds",
            SearchTerm::Group(SpeciesGroup::Birds).to_string()
        );
        assert_eq!("gen:foo", SearchTerm::Genus("foo".to_string()).to_string());
        assert_eq!(
            "gen:\"foo bar\"",
            SearchTerm::Genus("foo bar".to_string()).to_string()
        );
        assert_eq!("sp:foo", SearchTerm::Species("foo".to_string()).to_string());
        assert_eq!(
            "ssp:foo",
            SearchTerm::Subspecies("foo".to_string()).to_string()
        );
        assert_eq!(
            "ssp:\"foo bar\"",
            SearchTerm::Subspecies("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "rec:foo",
            SearchTerm::Recordist("foo".to_string()).to_string()
        );
        assert_eq!(
            "cnt:chile",
            SearchTerm::Country("chile".to_string()).to_string()
        );
        assert_eq!(
            "cnt:\"United States\"",
            SearchTerm::Country("United States".to_string()).to_string()
        );
        assert_eq!(
            "loc:foo",
            SearchTerm::Location("foo".to_string()).to_string()
        );
        assert_eq!(
            "loc:\"foo bar\"",
            SearchTerm::Location("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "rmk:foo",
            SearchTerm::Remarks("foo".to_string()).to_string()
        );
        assert_eq!(
            "rmk:\"I said \\\"hello\\\"\"",
            SearchTerm::Remarks("I said \"hello\"".to_string()).to_string()
        );
        assert_eq!("seen:yes", SearchTerm::Seen(true).to_string());
        assert_eq!("seen:no", SearchTerm::Seen(false).to_string());
        assert_eq!("playback:yes", SearchTerm::Playback(true).to_string());
        assert_eq!("playback:no", SearchTerm::Playback(false).to_string());
        assert_eq!("lat:-43.2", SearchTerm::Latitude(None, -43.20).to_string());
        assert_eq!(
            "lat:\"<-43.2\"",
            SearchTerm::Latitude(Some(Operator::LessThan), -43.20).to_string()
        );
        assert_eq!(
            "lat:\">-43.2\"",
            SearchTerm::Latitude(Some(Operator::GreaterThan), -43.20).to_string()
        );
        assert_eq!(
            "lon:95.234",
            SearchTerm::Longitude(None, 95.234).to_string()
        );
        assert_eq!(
            "box:1.1,2.2,3.3,4.4",
            SearchTerm::GeoBox(
                Point {
                    latitude: 1.1,
                    longitude: 2.2
                },
                Point {
                    latitude: 3.3,
                    longitude: 4.4
                }
            )
            .to_string()
        );
        assert_eq!(
            "type:call",
            SearchTerm::SoundType(SoundType::Call).to_string()
        );
        assert_eq!(
            "type:\"nocturnal flight call\"",
            SearchTerm::SoundType(SoundType::NocturnalFlightCall).to_string()
        );
        assert_eq!(
            "othertype:\"freeform text\"",
            SearchTerm::OtherType("freeform text".to_string()).to_string()
        );
        assert_eq!("sex:male", SearchTerm::Sex(Sex::Male).to_string());
        assert_eq!("sex:female", SearchTerm::Sex(Sex::Female).to_string());
        assert_eq!(
            "stage:adult",
            SearchTerm::LifeStage(LifeStage::Adult).to_string()
        );
        assert_eq!(
            "method:foo",
            SearchTerm::RecordingMethod("foo".to_string()).to_string()
        );
        assert_eq!("nr:12345", SearchTerm::RecordingId(12345).to_string());
        assert_eq!(
            "lic:by-nc-sa",
            SearchTerm::License(License::ByNcSa).to_string()
        );
        assert_eq!("q:A", SearchTerm::Quality(None, Quality::A).to_string());
        assert_eq!(
            "q:\"<A\"",
            SearchTerm::Quality(Some(Operator::LessThan), Quality::A).to_string()
        );
        assert_eq!(
            "len:10.563",
            SearchTerm::RecordingDuration(None, 10.563).to_string()
        );
        assert_eq!(
            "len:\">10.563\"",
            SearchTerm::RecordingDuration(Some(Operator::GreaterThan), 10.563).to_string()
        );
        assert_eq!(
            "len:\"=10.563\"",
            SearchTerm::RecordingDuration(Some(Operator::Matches), 10.563).to_string()
        );
        assert_eq!(
            "len:10.5-20.89",
            SearchTerm::RecordingDurationRange(10.5, 20.89).to_string()
        );
        assert_eq!(
            "area:america",
            SearchTerm::Area(WorldArea::America).to_string()
        );
        assert_eq!(
            "since:2025-03-12",
            SearchTerm::Since(jiff::civil::Date::new(2025, 3, 12).unwrap()).to_string()
        );
        assert_eq!(
            "year:2012",
            SearchTerm::RecordingYear(None, 2012).to_string()
        );
        assert_eq!(
            "year:\">2012\"",
            SearchTerm::RecordingYear(Some(Operator::GreaterThan), 2012).to_string()
        );
        assert_eq!("month:12", SearchTerm::RecordingMonth(None, 12).to_string());
        assert_eq!(
            "month:\"<12\"",
            SearchTerm::RecordingMonth(Some(Operator::LessThan), 12).to_string()
        );
        assert_eq!("temp:21.2", SearchTerm::Temperature(None, 21.2).to_string());
        assert_eq!(
            "temp:\"<21.2\"",
            SearchTerm::Temperature(Some(Operator::LessThan), 21.2).to_string()
        );
        assert_eq!("auto:no", SearchTerm::Automated(false).to_string());
        assert_eq!(
            "dvc:sony",
            SearchTerm::Device("sony".to_string()).to_string()
        );
        assert_eq!(
            "mic:\"Sennheiser ME62\"",
            SearchTerm::Microphone("Sennheiser ME62".to_string()).to_string()
        );
        assert_eq!("smp:48000", SearchTerm::SampleRate(None, 48000).to_string());
    }
}
