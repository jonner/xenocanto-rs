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

/// The main entry point for this library. a `Client` object represents a
/// configured client that can query the xeno-canto web server and return results.
pub struct Client {
    key: SecretString,
    client: reqwest::Client,
}

/// A type for building queries against the xeno-canto API.
pub struct QueryBuilder<'a> {
    fields: Vec<SearchField>,
    page_size: Option<u16>,
    client: &'a Client,
}

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

impl<'a> QueryBuilder<'a> {
    /// Build the query, send it to the xeno-canto web service and return the results
    pub async fn fetch_page(self, page: u16) -> Result<QueryResults, Error> {
        let search_fields: String = self
            .fields
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        tracing::debug!(search_fields);
        let mut params = vec![
            ("key", self.client.key.expose_secret().to_string()),
            ("query", search_fields),
        ];
        params.push(("page", page.to_string()));
        if let Some(limit) = self.page_size {
            params.push(("per_page", limit.to_string()))
        }
        let req = self.client.client.get(API_ENDPOINT).query(&params);
        let api_response = req.send().await?.text().await?;
        trace!(api_response);

        if let Ok(err) = serde_json::from_str::<ApiError>(&api_response) {
            Err(err.into())
        } else {
            serde_json::from_str::<QueryResults>(&api_response).map_err(Into::into)
        }
    }

    /// Add a new search field to the query
    pub fn and(mut self, field: SearchField) -> Self {
        self.fields.push(field);
        self
    }

    /// Specify the page size of the results that will be returned. Valid
    /// values range from 50 to 500
    pub fn page_size(mut self, size: u16) -> Self {
        self.page_size = Some(size);
        self
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

    /// Returns a new object for querying the xeno-canto web service with the
    /// given `field`. Build more complex queries using the [QueryBuilder] API.
    pub fn build_query(&'_ self, field: SearchField) -> QueryBuilder<'_> {
        QueryBuilder {
            client: self,
            fields: vec![field],
            page_size: None,
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
        serde_json::from_str::<QueryResults>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ApiError>(TEST_ERROR).expect("Failed to deserialize");
    }
    #[test]
    fn test_deserialization_response() {
        serde_json::from_str::<QueryResults>(TEST_SUCCESS).expect("Failed to deserialize");
        serde_json::from_str::<ApiError>(TEST_ERROR).expect("Failed to deserialize");
    }
}
