#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Scope {
    /// focal taxon
    pub taxon_coverage: String,
    /// whether all sounds of the focal taxon were identified
    pub completeness: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AnnotationSet {
    /// name of the set
    #[serde(rename = "set_name")]
    pub name: String,

    /// unique DOI/url of the annotation set (if available)
    #[serde(rename = "set_uri")]
    #[serde(with = "http_serde::uri")]
    pub uri: http::Uri,

    /// name of a findable resource from which the set of annotations is derived
    #[serde(rename = "set_source")]
    pub source_name: String,

    /// software/platform used to create the annotation(s)
    #[serde(rename = "annotation_software_name_and_version")]
    pub software: Option<String>,

    /// name of the person or organisation originally creating the set
    #[serde(rename = "set_creator")]
    pub creator: Option<String>,

    /// URI representing a person or organisation originally creating the set
    #[serde(rename = "set_creator_id")]
    #[serde(with = "http_serde::option::uri")]
    pub creator_id: Option<http::Uri>,

    /// list of legal owners of the resource
    pub owner: Option<String>,

    // pub uploader_id: String,
    /// Creative Commons copyright statement (from a fixed list)
    pub license: String,

    /// title or name of a project that contributed to the annotation set
    pub project_name: Option<String>,

    /// identifier for the project that contributed to the annotation set
    #[serde(with = "http_serde::option::uri")]
    pub project_uri: Option<http::Uri>,

    /// organisations or individuals who funded the creation of the resource
    pub funding: Option<String>,

    /// any relevant remarks regarding the set
    #[serde(rename = "set_remarks")]
    pub remarks: Option<String>,

    /// date on which the annotation set was imported to XC
    #[serde(rename = "set_import_date")]
    pub created: jiff::Timestamp,

    pub scope: Vec<Scope>,

    annotations: Vec<Annotation>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Annotation {
    /// unique id of the annotation in xeno-canto
    #[serde(rename = "annotation_xc_id")]
    pub id: String,

    /// Unique id of the annotation in the source database
    #[serde(rename = "annotation_source_id")]
    pub source_id: String,

    /// name of the original sound file that the annotation belongs to
    pub sound_file: String,

    /// XC number of the original sound file that the annotation belongs to
    pub xc_nr: String,

    /// name of the annotator
    pub annotator: Option<String>,

    /// XC id of the annotator (if known)
    pub annotator_xc_id: Option<String>,

    /// Highest frequency of the bounding box in Hertz
    pub frequency_high: Option<f64>,
    /// Lowest frequency of the bounding box in Hertz
    pub frequency_low: Option<f64>,

    /// start time (relative to recording) of the bounding box in seconds
    pub start_time: f64,
    /// end time (relative to recording) of the bounding box in seconds
    pub end_time: f64,

    /// scientific name of taxon, or indication sound is abiotic
    pub scientific_name: String,

    /// type of sound (e.g. 'song', 'call', 'rain'). From predefined list
    pub sound_type: Option<String>,

    // date on which the individual sound was annotated
    date_identified: Option<jiff::Timestamp>,

    /// sex of annotated individual. From a predefined list
    pub sex: Option<String>,

    /// life stage (e.g. 'adult') of annotated individual. From predefined list
    pub life_stage: Option<String>,

    /// was playback used to lure annotated individual
    pub playback_used: bool,

    /// collection date of annotated individual (studio recordings only)
    pub collection_date: jiff::Timestamp,

    /// registration number of collected & annotated individual
    pub collection_specimen: Option<String>,

    /// temperature recorded at the time of the annotation
    pub temperature: Option<f64>,

    /// SNR based on sound in bounding box
    pub signal_noise_ration: Option<f64>,

    /// Does an annotation overlap with a sound that has not been annotated?
    pub overlap: bool,

    #[serde(rename = "annotation_remarks")]
    pub remarks: Option<String>,
}
