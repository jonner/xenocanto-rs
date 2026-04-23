#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Error {
    pub code: String,
    pub message: String,
}
