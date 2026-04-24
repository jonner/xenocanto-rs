use std::{fmt::Display, str::FromStr};

/// possible values for the sex field in xeno-canto recordings
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    Uncertain,
    #[serde(other)]
    Unknown,
}

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_variant::to_variant_name(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl TryFrom<String> for Sex {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().parse()
    }
}

impl FromStr for Sex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" => Ok(Sex::Male),
            "female" => Ok(Sex::Female),
            "uncertain" => Ok(Sex::Uncertain),
            s => Err(format!("Invalid sex value: '{s}'")),
        }
    }
}

/// possible values for the life stage field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LifeStage {
    Adult,
    Juvenile,
    Nestling,
    Nymph,
    Subadult,
    Uncertain,
    #[serde(other)]
    Unknown,
}

impl FromStr for LifeStage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "adult" => Ok(LifeStage::Adult),
            "juvenile" => Ok(LifeStage::Juvenile),
            "nestling" => Ok(LifeStage::Nestling),
            "nymph" => Ok(LifeStage::Nymph),
            "subadult" => Ok(LifeStage::Subadult),
            "uncertain" => Ok(LifeStage::Uncertain),
            s => Err(format!("Invalid life stage value '{s}'")),
        }
    }
}

/// possible values for the recording group field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpeciesGroup {
    Birds,
    Grasshoppers,
    Bats,
    Frogs,
    #[serde(rename = "land mammals")]
    LandMammals,
    Soundscape,
    #[serde(other)]
    Unknown,
}

/// possible values for the license field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum License {
    ByNcSa,
    ByNcNd,
    BySa,
    ByNd,
    ByNc,
    By,
    Cc0,
    #[serde(other)]
    Unknown,
}

/// possible values for the world areas field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldArea {
    Africa,
    America,
    Asia,
    Australia,
    Europe,
}

/// possible values for the sound type field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum SoundType {
    #[serde(rename = "aberrant")]
    Aberrant,
    #[serde(rename = "advertisement call")]
    AdvertisementCall,
    #[serde(rename = "agonistic call")]
    AgonisticCall,
    #[serde(rename = "alarm call")]
    AlarmCall,
    #[serde(rename = "begging call")]
    BeggingCall,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "calling song")]
    CallingSong,
    #[serde(rename = "courtship song")]
    CourtshipSong,
    #[serde(rename = "dawn song")]
    DawnSong,
    #[serde(rename = "defensive call")]
    DefensiveCall,
    #[serde(rename = "distress call")]
    DistressCall,
    #[serde(rename = "disturbance song")]
    DisturbanceSong,
    #[serde(rename = "drumming")]
    Drumming,
    #[serde(rename = "duet")]
    Duet,
    #[serde(rename = "echolocation")]
    Echolocation,
    #[serde(rename = "feeding buzz")]
    FeedingBuzz,
    #[serde(rename = "female song")]
    FemaleSong,
    #[serde(rename = "flight call")]
    FlightCall,
    #[serde(rename = "flight song")]
    FlightSong,
    #[serde(rename = "imitation")]
    Imitation,
    #[serde(rename = "mating call")]
    MatingCall,
    #[serde(rename = "mechanical sound")]
    MechanicalSound,
    #[serde(rename = "nocturnal flight call")]
    NocturnalFlightCall,
    #[serde(rename = "release call")]
    ReleaseCall,
    #[serde(rename = "rivalry song")]
    RivalrySong,
    #[serde(rename = "searching song")]
    SearchingSong,
    #[serde(rename = "social call")]
    SocialCall,
    #[serde(rename = "song")]
    Song,
    #[serde(rename = "subsong")]
    Subsong,
    #[serde(rename = "territorial call")]
    TerritorialCall,
    #[serde(other)]
    Unknown,
}

/// possible values for the quality field in xeno-canto queries
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Quality {
    A,
    B,
    C,
    D,
    E,
    #[serde(other)]
    Unknown,
}
