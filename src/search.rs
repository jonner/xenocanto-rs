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
pub enum SearchField {
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

impl Display for SearchField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchField::Group(species_group) => {
                write!(
                    f,
                    "grp:{}",
                    escape(serde_variant::to_variant_name(&species_group).unwrap())
                )
            }
            SearchField::Genus(val) => write!(f, "gen:{}", escape(val)),
            SearchField::Species(val) => write!(f, "sp:{}", escape(val)),
            SearchField::Subspecies(val) => write!(f, "ssp:{}", escape(val)),
            SearchField::Recordist(val) => write!(f, "rec:{}", escape(val)),
            SearchField::Country(val) => write!(f, "cnt:{}", escape(val)),
            SearchField::Location(val) => write!(f, "loc:{}", escape(val)),
            SearchField::Remarks(val) => write!(f, "rmk:{}", escape(val)),
            SearchField::Seen(val) => write!(f, "seen:{}", yesno(*val)),
            SearchField::Playback(val) => write!(f, "playback:{}", yesno(*val)),
            SearchField::Latitude(operator, val) => {
                write!(
                    f,
                    "lat:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchField::Longitude(operator, val) => {
                write!(
                    f,
                    "lon:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchField::GeoBox(p1, p2) => write!(
                f,
                "box:{},{},{},{}",
                p1.latitude, p1.longitude, p2.latitude, p2.longitude
            ),
            SearchField::SoundType(sound_type) => {
                write!(
                    f,
                    "type:{}",
                    escape(serde_variant::to_variant_name(sound_type).unwrap())
                )
            }
            SearchField::OtherType(val) => write!(f, "othertype:{}", escape(val)),
            SearchField::Sex(sex) => write!(
                f,
                "sex:{}",
                escape(serde_variant::to_variant_name(sex).unwrap())
            ),
            SearchField::LifeStage(life_stage) => write!(
                f,
                "stage:{}",
                escape(serde_variant::to_variant_name(life_stage).unwrap())
            ),
            SearchField::RecordingMethod(val) => write!(f, "method:{}", escape(val)),
            SearchField::RecordingId(id) => write!(f, "nr:{id}"),
            SearchField::License(license) => write!(
                f,
                "lic:{}",
                escape(serde_variant::to_variant_name(license).unwrap())
            ),
            SearchField::Quality(operator, quality) => {
                write!(
                    f,
                    "q:{}",
                    escape_with_operator(
                        serde_variant::to_variant_name(quality).unwrap(),
                        operator.as_ref()
                    )
                )
            }
            SearchField::RecordingDuration(operator, duration) => write!(
                f,
                "len:{}",
                escape_with_operator(&duration.to_string(), operator.as_ref())
            ),
            SearchField::RecordingDurationRange(from, to) => write!(f, "len:{from}-{to}"),
            SearchField::Area(world_area) => {
                write!(
                    f,
                    "area:{}",
                    escape(serde_variant::to_variant_name(world_area).unwrap())
                )
            }
            SearchField::Since(date) => write!(f, "since:{}", date.strftime("%F")),
            SearchField::RecordingYear(operator, year) => {
                write!(
                    f,
                    "year:{}",
                    escape_with_operator(&year.to_string(), operator.as_ref())
                )
            }
            SearchField::RecordingMonth(operator, month) => {
                write!(
                    f,
                    "month:{}",
                    escape_with_operator(&month.to_string(), operator.as_ref())
                )
            }
            SearchField::Temperature(operator, temp) => write!(
                f,
                "temp:{}",
                escape_with_operator(&temp.to_string(), operator.as_ref())
            ),
            SearchField::Automated(val) => write!(f, "auto:{}", yesno(*val)),
            SearchField::Device(val) => write!(f, "dvc:{}", escape(val)),
            SearchField::Microphone(val) => write!(f, "mic:{}", escape(val)),
            SearchField::SampleRate(operator, val) => write!(
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
            SearchField::Group(SpeciesGroup::LandMammals).to_string()
        );
        assert_eq!(
            "grp:birds",
            SearchField::Group(SpeciesGroup::Birds).to_string()
        );
        assert_eq!("gen:foo", SearchField::Genus("foo".to_string()).to_string());
        assert_eq!(
            "gen:\"foo bar\"",
            SearchField::Genus("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "sp:foo",
            SearchField::Species("foo".to_string()).to_string()
        );
        assert_eq!(
            "ssp:foo",
            SearchField::Subspecies("foo".to_string()).to_string()
        );
        assert_eq!(
            "ssp:\"foo bar\"",
            SearchField::Subspecies("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "rec:foo",
            SearchField::Recordist("foo".to_string()).to_string()
        );
        assert_eq!(
            "cnt:chile",
            SearchField::Country("chile".to_string()).to_string()
        );
        assert_eq!(
            "cnt:\"United States\"",
            SearchField::Country("United States".to_string()).to_string()
        );
        assert_eq!(
            "loc:foo",
            SearchField::Location("foo".to_string()).to_string()
        );
        assert_eq!(
            "loc:\"foo bar\"",
            SearchField::Location("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "rmk:foo",
            SearchField::Remarks("foo".to_string()).to_string()
        );
        assert_eq!(
            "rmk:\"I said \\\"hello\\\"\"",
            SearchField::Remarks("I said \"hello\"".to_string()).to_string()
        );
        assert_eq!("seen:yes", SearchField::Seen(true).to_string());
        assert_eq!("seen:no", SearchField::Seen(false).to_string());
        assert_eq!("playback:yes", SearchField::Playback(true).to_string());
        assert_eq!("playback:no", SearchField::Playback(false).to_string());
        assert_eq!("lat:-43.2", SearchField::Latitude(None, -43.20).to_string());
        assert_eq!(
            "lat:\"<-43.2\"",
            SearchField::Latitude(Some(Operator::LessThan), -43.20).to_string()
        );
        assert_eq!(
            "lat:\">-43.2\"",
            SearchField::Latitude(Some(Operator::GreaterThan), -43.20).to_string()
        );
        assert_eq!(
            "lon:95.234",
            SearchField::Longitude(None, 95.234).to_string()
        );
        assert_eq!(
            "box:1.1,2.2,3.3,4.4",
            SearchField::GeoBox(
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
            SearchField::SoundType(SoundType::Call).to_string()
        );
        assert_eq!(
            "type:\"nocturnal flight call\"",
            SearchField::SoundType(SoundType::NocturnalFlightCall).to_string()
        );
        assert_eq!(
            "othertype:\"freeform text\"",
            SearchField::OtherType("freeform text".to_string()).to_string()
        );
        assert_eq!("sex:male", SearchField::Sex(Sex::Male).to_string());
        assert_eq!("sex:female", SearchField::Sex(Sex::Female).to_string());
        assert_eq!(
            "stage:adult",
            SearchField::LifeStage(LifeStage::Adult).to_string()
        );
        assert_eq!(
            "method:foo",
            SearchField::RecordingMethod("foo".to_string()).to_string()
        );
        assert_eq!("nr:12345", SearchField::RecordingId(12345).to_string());
        assert_eq!(
            "lic:by-nc-sa",
            SearchField::License(License::ByNcSa).to_string()
        );
        assert_eq!("q:A", SearchField::Quality(None, Quality::A).to_string());
        assert_eq!(
            "q:\"<A\"",
            SearchField::Quality(Some(Operator::LessThan), Quality::A).to_string()
        );
        assert_eq!(
            "len:10.563",
            SearchField::RecordingDuration(None, 10.563).to_string()
        );
        assert_eq!(
            "len:\">10.563\"",
            SearchField::RecordingDuration(Some(Operator::GreaterThan), 10.563).to_string()
        );
        assert_eq!(
            "len:\"=10.563\"",
            SearchField::RecordingDuration(Some(Operator::Matches), 10.563).to_string()
        );
        assert_eq!(
            "len:10.5-20.89",
            SearchField::RecordingDurationRange(10.5, 20.89).to_string()
        );
        assert_eq!(
            "area:america",
            SearchField::Area(WorldArea::America).to_string()
        );
        assert_eq!(
            "since:2025-03-12",
            SearchField::Since(jiff::civil::Date::new(2025, 3, 12).unwrap()).to_string()
        );
        assert_eq!(
            "year:2012",
            SearchField::RecordingYear(None, 2012).to_string()
        );
        assert_eq!(
            "year:\">2012\"",
            SearchField::RecordingYear(Some(Operator::GreaterThan), 2012).to_string()
        );
        assert_eq!(
            "month:12",
            SearchField::RecordingMonth(None, 12).to_string()
        );
        assert_eq!(
            "month:\"<12\"",
            SearchField::RecordingMonth(Some(Operator::LessThan), 12).to_string()
        );
        assert_eq!(
            "temp:21.2",
            SearchField::Temperature(None, 21.2).to_string()
        );
        assert_eq!(
            "temp:\"<21.2\"",
            SearchField::Temperature(Some(Operator::LessThan), 21.2).to_string()
        );
        assert_eq!("auto:no", SearchField::Automated(false).to_string());
        assert_eq!(
            "dvc:sony",
            SearchField::Device("sony".to_string()).to_string()
        );
        assert_eq!(
            "mic:\"Sennheiser ME62\"",
            SearchField::Microphone("Sennheiser ME62".to_string()).to_string()
        );
        assert_eq!(
            "smp:48000",
            SearchField::SampleRate(None, 48000).to_string()
        );
    }
}
