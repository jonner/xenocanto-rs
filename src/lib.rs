use std::{fmt::Display, str::FromStr};

use secrecy::{ExposeSecret, SecretString};

use crate::{error::Error, recording::RecordingSet};

pub mod annota;
pub mod error;
pub mod recording;
pub mod request;
pub mod search;
pub mod util;

pub const API_ENDPOINT: &str = "https://xeno-canto.org/api/3/recordings";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
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
            _ => Err("Invalid sex value".to_string()),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LifeStage {
    Adult,
    Juvenile,
    Nestling,
    Nymph,
    Subadult,
    #[serde(other)]
    Unknown,
}

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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldArea {
    Africa,
    America,
    Asia,
    Australia,
    Europe,
}

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

pub struct Service {
    pub key: SecretString,
    client: reqwest::Client,
}

impl Service {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn request<Q>(&self, query: Q) -> reqwest::Result<Response>
    where
        Q: IntoIterator<Item = search::Term>,
    {
        let search_terms: String = query
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let req = self
            .client
            .get(API_ENDPOINT)
            .query(&[("key", self.key.expose_secret()), ("query", &search_terms)]);
        req.send().await?.json().await
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success(RecordingSet),
    Err(ErrorResponse),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    error: Error,
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_SUCCESS: &str = r#"{
    "numRecordings": "1",
    "numSpecies": "1",
    "page": 1,
    "numPages": 1,
    "recordings": [
        {
            "id": "254462",
            "gen": "Passerina",
            "sp": "amoena",
            "ssp": "",
            "grp": "birds",
            "en": "Lazuli Bunting",
            "rec": "Jonathon Jongsma",
            "cnt": "United States",
            "loc": "Prairie Trail, Custer State Park, South Dakota",
            "lat": "43.6571",
            "lon": "-103.4143",
            "alt": "1300",
            "type": "song",
            "sex": "",
            "stage": "",
            "method": "field recording",
            "url": "https://xeno-canto.org/254462",
            "file": "https://xeno-canto.org/254462/download",
            "file-name": "XC254462-JMJ-20150623-031744-150623_16-USA-SD-CusterStatePark-LABU.mp3",
            "sono": {
                "small": "https://xeno-canto.org/sounds/spectrograms/OJMFAOUBDU/254462/grey-small.png",
                "med": "https://xeno-canto.org/sounds/spectrograms/OJMFAOUBDU/254462/grey-medium.png",
                "large": "https://xeno-canto.org/sounds/spectrograms/OJMFAOUBDU/254462/colour.png",
                "full": null
            },
            "osci": {
                "small": null,
                "med": null,
                "large": null
            },
            "lic": "https://creativecommons.org/licenses/by-sa/4.0/",
            "q": "A",
            "length": "4:13",
            "time": "07:17",
            "date": "2015-06-23",
            "uploaded": "2015-06-29",
            "also": [
                "Pipilo maculatus"
            ],
            "rmk": "Songs from a bird perched atop a ponderosa pine at the edge of prairie. Same bird as XC254461, after flying to a different perch.\r\n\r\nFiltered lightly below 1kHz.",
            "animal-seen": "yes",
            "playback-used": "no",
            "temp": null,
            "regnr": "",
            "auto": "no",
            "dvc": "",
            "mic": "",
            "smp": "48000",
            "annotation-set": []
        }
    ]
}
"#;
    const TEST_ERROR: &str = r#"{
    "error": {
        "code": "missing_parameter",
        "message": "no query specified"
    }
}"#;
    #[test]
    fn test_deserialization_nonresponse() {
        serde_json::from_str::<RecordingSet>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ErrorResponse>(TEST_ERROR).expect("Failed to deserialize");
    }
    #[test]
    fn test_deserialization_response() {
        serde_json::from_str::<Response>(TEST_ERROR).expect("Failed to deserialize");
        serde_json::from_str::<Response>(TEST_SUCCESS).expect("Failed to deserialize");
    }
}
