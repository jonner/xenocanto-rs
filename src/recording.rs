use crate::{LifeStage, Quality, Sex, SoundType, SpeciesGroup, annota::AnnotationSet};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Sonograms {
    #[serde(with = "http_serde::option::uri")]
    pub small: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    pub med: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    pub large: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    pub full: Option<http::Uri>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Oscillograms {
    #[serde(with = "http_serde::option::uri")]
    pub small: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    pub med: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    pub large: Option<http::Uri>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Recording {
    /// the catalogue number of the recording on xeno-canto
    #[serde(deserialize_with = "crate::util::deserialize_number_from_string")]
    pub id: u64,

    /// the generic name of the species
    #[serde(rename = "gen")]
    pub genus: String,

    /// the specific name (epithet) of the species
    #[serde(rename = "sp")]
    pub species: String,

    /// the subspecies name (subspecific epithet)
    #[serde(rename = "ssp", deserialize_with = "crate::util::empty_string_as_none")]
    pub subspecies: Option<String>,

    /// the group to which the species belongs (birds, grasshoppers, bats)
    #[serde(rename = "grp")]
    pub group: Option<SpeciesGroup>,

    /// the English name of the species
    #[serde(rename = "en")]
    pub english_name: String,

    /// the name of the recordist
    #[serde(rename = "rec")]
    pub recordist: String,

    /// the country where the recording was made
    #[serde(rename = "cnt")]
    pub country: String,

    /// the name of the locality
    #[serde(rename = "loc")]
    pub location: String,

    /// the latitude of the recording in decimal coordinates
    #[serde(
        rename = "lat",
        deserialize_with = "crate::util::maybe_deserialize_number_from_string"
    )]
    pub latitude: Option<f64>,

    /// the longitude of the recording in decimal coordinates
    #[serde(
        rename = "lon",
        deserialize_with = "crate::util::maybe_deserialize_number_from_string"
    )]
    pub longitude: Option<f64>,

    /// the sound type of the recording (combining both predefined terms such as 'call' or 'song' and additional free text options)
    #[serde(rename = "type")]
    pub sound_type: SoundType,

    /// the sex of the animal
    #[serde(deserialize_with = "crate::util::deserialize_csv_to_vec")]
    pub sex: Vec<Sex>,

    /// the life stage of the animal (adult, juvenile, etc.)
    #[serde(
        rename = "stage",
        deserialize_with = "crate::util::deserialize_csv_to_vec"
    )]
    pub life_stage: Vec<LifeStage>,

    /// the recording method (field recording, in the hand, etc.)
    pub method: String,

    /// the URL specifying the details of this recording
    #[serde(rename = "url", with = "http_serde::uri")]
    pub info_uri: http::Uri,

    /// the URL to the audio file
    #[serde(rename = "file", with = "http_serde::uri")]
    pub file_uri: http::Uri,

    /// the original file name of the audio file
    #[serde(rename = "file-name")]
    pub file_name: String,

    /// an object with the urls to the four versions of sonograms
    #[serde(rename = "sono")]
    pub sonograms: Sonograms,

    /// an object with the urls to the three versions of oscillograms
    #[serde(rename = "osci")]
    pub oscillograms: Oscillograms,

    /// the URL describing the license of this recording
    #[serde(rename = "lic", with = "http_serde::uri")]
    pub license: http::Uri,

    /// the current quality rating for the recording
    #[serde(rename = "q")]
    pub quality: Option<Quality>,

    /// the length of the recording in minutes
    #[serde(deserialize_with = "crate::util::deserialize_duration")]
    pub length: jiff::SignedDuration,

    /// the time of day that the recording was made
    #[serde(rename = "time", deserialize_with = "crate::util::permissive_time")]
    pub recording_time: Option<jiff::civil::Time>,

    /// the date that the recording was made
    #[serde(rename = "date", deserialize_with = "crate::util::permissive_date")]
    pub recording_date: Option<jiff::civil::Date>,

    /// temperature during recording (applicable to specific groups only)
    #[serde(
        rename = "temp",
        deserialize_with = "crate::util::maybe_deserialize_number_from_string"
    )]
    pub recording_temperature: Option<f64>,

    /// the date that the recording was uploaded to xeno-canto
    #[serde(rename = "uploaded")]
    pub upload_date: jiff::civil::Date,

    /// an array with the identified background species in the recording
    #[serde(rename = "also")]
    pub background_species: Vec<String>,

    /// additional remarks by the recordist
    #[serde(rename = "rmk")]
    pub remarks: String,

    /// was the recorded animal seen?
    #[serde(
        rename = "animal-seen",
        deserialize_with = "crate::util::maybe_yes_no_to_bool"
    )]
    pub seen: Option<bool>,

    /// was playback used to lure the animal?
    #[serde(
        rename = "playback-used",
        deserialize_with = "crate::util::maybe_yes_no_to_bool"
    )]
    pub playback_used: Option<bool>,

    /// automatic (non-supervised) recording?
    #[serde(
        rename = "auto",
        deserialize_with = "crate::util::maybe_yes_no_to_bool"
    )]
    pub automated_recording: Option<bool>,

    /// registration number of specimen (when collected)
    #[serde(
        rename = "regnr",
        deserialize_with = "crate::util::empty_string_as_none"
    )]
    pub registration_number: Option<String>,

    /// recording device used
    #[serde(rename = "dvc", deserialize_with = "crate::util::empty_string_as_none")]
    pub recording_device: Option<String>,

    /// microphone used
    #[serde(rename = "mic", deserialize_with = "crate::util::empty_string_as_none")]
    pub microphone: Option<String>,

    /// sample rate
    #[serde(
        rename = "smp",
        deserialize_with = "crate::util::deserialize_number_from_string"
    )]
    pub sample_rate: u64,

    annotation_set: Option<AnnotationSet>,
}
