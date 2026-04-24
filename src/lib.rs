use secrecy::{ExposeSecret, SecretString};
use tracing::trace;

use crate::{
    error::{ApiError, Error},
    recording::RecordingSet,
    search::SearchTerm,
};

pub mod annota;
pub mod error;
pub mod recording;
pub mod request;
pub mod search;
mod types;
pub mod util;

pub use types::*;

pub const API_ENDPOINT: &str = "https://xeno-canto.org/api/3/recordings";

pub struct Service {
    pub key: SecretString,
    client: reqwest::Client,
}

pub struct QueryBuilder<'a> {
    terms: Vec<SearchTerm>,
    page: Option<u64>,
    limit: Option<u64>,
    service: &'a Service,
}

impl<'a> QueryBuilder<'a> {
    pub async fn send(self) -> Result<RecordingSet, Error> {
        let search_terms: String = self
            .terms
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        tracing::debug!(search_terms);
        let mut params = vec![
            ("key", self.service.key.expose_secret().to_string()),
            ("query", search_terms),
        ];
        if let Some(page) = self.page {
            params.push(("page", page.to_string()))
        }
        if let Some(limit) = self.limit {
            params.push(("per_page", limit.to_string()))
        }
        let req = self.service.client.get(API_ENDPOINT).query(&params);
        let api_response = req.send().await?.text().await?;
        trace!(api_response);

        if let Ok(err) = serde_json::from_str::<ApiError>(&api_response) {
            Err(err.into())
        } else {
            serde_json::from_str::<RecordingSet>(&api_response).map_err(Into::into)
        }
    }

    pub fn add_term(mut self, term: SearchTerm) -> Self {
        self.terms.push(term);
        self
    }

    pub fn limit(mut self, lim: u64) -> Self {
        self.limit = Some(lim);
        self
    }

    pub fn page(mut self, pg: u64) -> Self {
        self.page = Some(pg);
        self
    }
}

impl Service {
    pub fn with_key(key: &str) -> Self {
        Self {
            key: key.into(),
            client: reqwest::Client::new(),
        }
    }

    pub fn query(&'_ self) -> QueryBuilder<'_> {
        QueryBuilder {
            service: self,
            terms: Default::default(),
            page: None,
            limit: None,
        }
    }
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
    "error": "client_error",
    "message": "Missing or invalid 'key' parameter. Visit https://xeno-canto.org/account to retrieve your API key."
}"#;
    #[test]
    fn test_deserialization_direct() {
        serde_json::from_str::<RecordingSet>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ApiError>(TEST_ERROR).expect("Failed to deserialize");
    }
    #[test]
    fn test_deserialization_response() {
        serde_json::from_str::<RecordingSet>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ApiError>(TEST_ERROR).expect("Failed to deserialize");
    }
}
