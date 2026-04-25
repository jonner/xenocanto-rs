use secrecy::{ExposeSecret, SecretString};
use tracing::trace;

pub mod annota;
mod error;
pub mod recording;
pub mod search;
mod types;
mod util;

pub use error::*;
pub use recording::Recording;
pub use search::SearchField;
pub use types::*;

const API_ENDPOINT: &str = "https://xeno-canto.org/api/3/recordings";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct QueryResults {
    #[serde(
        rename = "numRecordings",
        deserialize_with = "crate::util::deserialize_number_from_string"
    )]
    pub total_recordings: u64,
    #[serde(
        rename = "numSpecies",
        deserialize_with = "crate::util::deserialize_number_from_string"
    )]
    pub num_species: u64,
    pub page: u64,
    #[serde(rename = "numPages")]
    pub total_pages: u64,
    pub recordings: Vec<Recording>,
}

/// The main entry point for this library. a `Client` object represents a
/// configured client that can query the xeno-canto web server and return results.
pub struct Client {
    key: SecretString,
    client: reqwest::Client,
}

#[derive(Debug, Default)]
pub struct Query {
    fields: Vec<SearchField>,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field(mut self, field: SearchField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn params(&self) -> String {
        self.fields
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn parse_response(api_response: &str) -> Result<QueryResults, Error> {
    if let Ok(err) = serde_json::from_str::<ApiError>(api_response) {
        Err(err.into())
    } else {
        serde_json::from_str::<QueryResults>(api_response).map_err(Into::into)
    }
}

impl Client {
    /// Create a new xeno-canto webservice client with the specified API Key
    pub fn with_key(key: &str) -> Self {
        Self {
            key: key.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Build the query, send it to the xeno-canto web service and return the results
    pub async fn fetch(
        self,
        query: &Query,
        page: Option<u16>,
        page_size: Option<u16>,
    ) -> Result<QueryResults, Error> {
        let mut params = vec![("key", self.key.expose_secret().to_string())];
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("per_page", page_size.to_string()));
        }
        params.push(("query", query.params()));
        let req = self.client.get(API_ENDPOINT).query(&params);
        let api_response = req.send().await?.text().await?;
        trace!(api_response);

        parse_response(&api_response)
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
            "sex": "male, female",
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
        serde_json::from_str::<QueryResults>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ApiError>(TEST_ERROR).expect("Failed to deserialize");
    }
    #[test]
    fn test_deserialization_response() {
        assert!(parse_response(TEST_SUCCESS).is_ok());
        assert!(parse_response(TEST_ERROR).is_err());
    }
    #[test]
    fn test_parse_sex() {
        let res = parse_response(TEST_SUCCESS).expect("Failed to parse response");
        let rec = res.recordings.first().expect("doesn't contain a recording");
        assert_eq!(rec.sex, &[Sex::Male, Sex::Female]);
    }
}
