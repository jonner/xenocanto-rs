use std::fmt::Display;

use crate::{License, LifeStage, Quality, Sex, SoundType, SpeciesGroup, WorldArea};

pub enum Term {
    Plain(String),
    Tagged(SearchTag),
}

impl From<SearchTag> for Term {
    fn from(value: SearchTag) -> Self {
        Self::Tagged(value)
    }
}

impl From<String> for Term {
    fn from(value: String) -> Self {
        Self::Plain(value)
    }
}

impl From<&str> for Term {
    fn from(value: &str) -> Self {
        Self::Plain(value.to_string())
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Plain(s) => write!(f, "{s}"),
            Term::Tagged(search_tag) => write!(f, "{search_tag}"),
        }
    }
}

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
pub enum SearchTag {
    Group(SpeciesGroup),
    Genus(String),
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

impl Display for SearchTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchTag::Group(species_group) => {
                write!(
                    f,
                    "grp:{}",
                    escape(serde_variant::to_variant_name(&species_group).unwrap())
                )
            }
            SearchTag::Genus(val) => write!(f, "gen:{}", escape(val)),
            SearchTag::Subspecies(val) => write!(f, "ssp:{}", escape(val)),
            SearchTag::Recordist(val) => write!(f, "rec:{}", escape(val)),
            SearchTag::Country(val) => write!(f, "cnt:{}", escape(val)),
            SearchTag::Location(val) => write!(f, "loc:{}", escape(val)),
            SearchTag::Remarks(val) => write!(f, "rmk:{}", escape(val)),
            SearchTag::Seen(val) => write!(f, "seen:{}", yesno(*val)),
            SearchTag::Playback(val) => write!(f, "playback:{}", yesno(*val)),
            SearchTag::Latitude(operator, val) => {
                write!(
                    f,
                    "lat:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchTag::Longitude(operator, val) => {
                write!(
                    f,
                    "lon:{}",
                    escape_with_operator(&val.to_string(), operator.as_ref())
                )
            }
            SearchTag::GeoBox(p1, p2) => write!(
                f,
                "box:{},{},{},{}",
                p1.latitude, p1.longitude, p2.latitude, p2.longitude
            ),
            SearchTag::SoundType(sound_type) => {
                write!(
                    f,
                    "type:{}",
                    escape(serde_variant::to_variant_name(sound_type).unwrap())
                )
            }
            SearchTag::OtherType(val) => write!(f, "othertype:{}", escape(val)),
            SearchTag::Sex(sex) => write!(
                f,
                "sex:{}",
                escape(serde_variant::to_variant_name(sex).unwrap())
            ),
            SearchTag::LifeStage(life_stage) => write!(
                f,
                "stage:{}",
                escape(serde_variant::to_variant_name(life_stage).unwrap())
            ),
            SearchTag::RecordingMethod(val) => write!(f, "method:{}", escape(val)),
            SearchTag::RecordingId(id) => write!(f, "nr:{id}"),
            SearchTag::License(license) => write!(
                f,
                "lic:{}",
                escape(serde_variant::to_variant_name(license).unwrap())
            ),
            SearchTag::Quality(operator, quality) => {
                write!(
                    f,
                    "q:{}",
                    escape_with_operator(
                        serde_variant::to_variant_name(quality).unwrap(),
                        operator.as_ref()
                    )
                )
            }
            SearchTag::RecordingDuration(operator, duration) => write!(
                f,
                "len:{}",
                escape_with_operator(&duration.to_string(), operator.as_ref())
            ),
            SearchTag::RecordingDurationRange(from, to) => write!(f, "len:{from}-{to}"),
            SearchTag::Area(world_area) => {
                write!(
                    f,
                    "area:{}",
                    escape(serde_variant::to_variant_name(world_area).unwrap())
                )
            }
            SearchTag::Since(date) => write!(f, "since:{}", date.strftime("%F")),
            SearchTag::RecordingYear(operator, year) => {
                write!(
                    f,
                    "year:{}",
                    escape_with_operator(&year.to_string(), operator.as_ref())
                )
            }
            SearchTag::RecordingMonth(operator, month) => {
                write!(
                    f,
                    "month:{}",
                    escape_with_operator(&month.to_string(), operator.as_ref())
                )
            }
            SearchTag::Temperature(operator, temp) => write!(
                f,
                "temp:{}",
                escape_with_operator(&temp.to_string(), operator.as_ref())
            ),
            SearchTag::Automated(val) => write!(f, "auto:{}", yesno(*val)),
            SearchTag::Device(val) => write!(f, "dvc:{}", escape(val)),
            SearchTag::Microphone(val) => write!(f, "mic:{}", escape(val)),
            SearchTag::SampleRate(operator, val) => write!(
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
            SearchTag::Group(SpeciesGroup::LandMammals).to_string()
        );
        assert_eq!(
            "grp:birds",
            SearchTag::Group(SpeciesGroup::Birds).to_string()
        );
        assert_eq!("gen:foo", SearchTag::Genus("foo".to_string()).to_string());
        assert_eq!(
            "gen:\"foo bar\"",
            SearchTag::Genus("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "ssp:foo",
            SearchTag::Subspecies("foo".to_string()).to_string()
        );
        assert_eq!(
            "ssp:\"foo bar\"",
            SearchTag::Subspecies("foo bar".to_string()).to_string()
        );
        assert_eq!(
            "rec:foo",
            SearchTag::Recordist("foo".to_string()).to_string()
        );
        assert_eq!(
            "cnt:chile",
            SearchTag::Country("chile".to_string()).to_string()
        );
        assert_eq!(
            "cnt:\"United States\"",
            SearchTag::Country("United States".to_string()).to_string()
        );
        assert_eq!(
            "loc:foo",
            SearchTag::Location("foo".to_string()).to_string()
        );
        assert_eq!(
            "loc:\"foo bar\"",
            SearchTag::Location("foo bar".to_string()).to_string()
        );
        assert_eq!("rmk:foo", SearchTag::Remarks("foo".to_string()).to_string());
        assert_eq!(
            "rmk:\"I said \\\"hello\\\"\"",
            SearchTag::Remarks("I said \"hello\"".to_string()).to_string()
        );
        assert_eq!("seen:yes", SearchTag::Seen(true).to_string());
        assert_eq!("seen:no", SearchTag::Seen(false).to_string());
        assert_eq!("playback:yes", SearchTag::Playback(true).to_string());
        assert_eq!("playback:no", SearchTag::Playback(false).to_string());
        assert_eq!("lat:-43.2", SearchTag::Latitude(None, -43.20).to_string());
        assert_eq!(
            "lat:\"<-43.2\"",
            SearchTag::Latitude(Some(Operator::LessThan), -43.20).to_string()
        );
        assert_eq!(
            "lat:\">-43.2\"",
            SearchTag::Latitude(Some(Operator::GreaterThan), -43.20).to_string()
        );
        assert_eq!("lon:95.234", SearchTag::Longitude(None, 95.234).to_string());
        assert_eq!(
            "box:1.1,2.2,3.3,4.4",
            SearchTag::GeoBox(
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
            SearchTag::SoundType(SoundType::Call).to_string()
        );
        assert_eq!(
            "type:\"nocturnal flight call\"",
            SearchTag::SoundType(SoundType::NocturnalFlightCall).to_string()
        );
        assert_eq!(
            "othertype:\"freeform text\"",
            SearchTag::OtherType("freeform text".to_string()).to_string()
        );
        assert_eq!("sex:male", SearchTag::Sex(Sex::Male).to_string());
        assert_eq!("sex:female", SearchTag::Sex(Sex::Female).to_string());
        assert_eq!(
            "stage:adult",
            SearchTag::LifeStage(LifeStage::Adult).to_string()
        );
        assert_eq!(
            "method:foo",
            SearchTag::RecordingMethod("foo".to_string()).to_string()
        );
        assert_eq!("nr:12345", SearchTag::RecordingId(12345).to_string());
        assert_eq!(
            "lic:by-nc-sa",
            SearchTag::License(License::ByNcSa).to_string()
        );
        assert_eq!("q:A", SearchTag::Quality(None, Quality::A).to_string());
        assert_eq!(
            "q:\"<A\"",
            SearchTag::Quality(Some(Operator::LessThan), Quality::A).to_string()
        );
        assert_eq!(
            "len:10.563",
            SearchTag::RecordingDuration(None, 10.563).to_string()
        );
        assert_eq!(
            "len:\">10.563\"",
            SearchTag::RecordingDuration(Some(Operator::GreaterThan), 10.563).to_string()
        );
        assert_eq!(
            "len:\"=10.563\"",
            SearchTag::RecordingDuration(Some(Operator::Matches), 10.563).to_string()
        );
        assert_eq!(
            "len:10.5-20.89",
            SearchTag::RecordingDurationRange(10.5, 20.89).to_string()
        );
        assert_eq!(
            "area:america",
            SearchTag::Area(WorldArea::America).to_string()
        );
        assert_eq!(
            "since:2025-03-12",
            SearchTag::Since(jiff::civil::Date::new(2025, 3, 12).unwrap()).to_string()
        );
        assert_eq!(
            "year:2012",
            SearchTag::RecordingYear(None, 2012).to_string()
        );
        assert_eq!(
            "year:\">2012\"",
            SearchTag::RecordingYear(Some(Operator::GreaterThan), 2012).to_string()
        );
        assert_eq!("month:12", SearchTag::RecordingMonth(None, 12).to_string());
        assert_eq!(
            "month:\"<12\"",
            SearchTag::RecordingMonth(Some(Operator::LessThan), 12).to_string()
        );
        assert_eq!("temp:21.2", SearchTag::Temperature(None, 21.2).to_string());
        assert_eq!(
            "temp:\"<21.2\"",
            SearchTag::Temperature(Some(Operator::LessThan), 21.2).to_string()
        );
        assert_eq!("auto:no", SearchTag::Automated(false).to_string());
        assert_eq!(
            "dvc:sony",
            SearchTag::Device("sony".to_string()).to_string()
        );
        assert_eq!(
            "mic:\"Sennheiser ME62\"",
            SearchTag::Microphone("Sennheiser ME62".to_string()).to_string()
        );
        assert_eq!("smp:48000", SearchTag::SampleRate(None, 48000).to_string());
    }
}
