use std::path::PathBuf;

use clap::Parser;

use xenocanto::{Query, SearchField};

#[derive(clap::Parser)]
#[command(group(
    clap::ArgGroup::new("field")
        .required(true)
        .multiple(true),
))]
struct Args {
    #[arg(short, long)]
    out_dir: PathBuf,
    #[arg(long)]
    page: Option<u16>,
    #[arg(long)]
    page_size: Option<u16>,
    #[arg(long, group = "field")]
    // group: SpeciesGroup,
    #[arg(short, long, group = "field")]
    genus: Option<String>,
    #[arg(short, long, group = "field")]
    species: Option<String>,
    #[arg(long, group = "field")]
    subspecies: Option<String>,
    #[arg(short, long, group = "field")]
    recordist: Option<String>,
    #[arg(short, long, group = "field")]
    country: Option<String>,
    #[arg(short, long, group = "field")]
    location: Option<String>,
    #[arg(long, group = "field")]
    remarks: Option<String>,
    #[arg(long, group = "field")]
    seen: Option<bool>,
    #[arg(long, group = "field")]
    playback: Option<bool>,
    // Latitude(Option<Operator>, f64),
    // Longitude(Option<Operator>, f64),
    // GeoBox(Point, Point),
    // #[arg(long, group="field")]
    // sound_type: Option<SoundType>,
    // OtherType(String),
    // #[arg(long, group="field")]
    // sex: Option<Sex>,
    // LifeStage(LifeStage),
    // RecordingMethod(String),
    #[arg(short = 'i', long, group = "field")]
    id: Option<u64>,
    // License(License),
    // Quality(Option<Operator>, Quality),
    // RecordingDuration(Option<Operator>, f64),
    // RecordingDurationRange(f64, f64),
    // Area(WorldArea),
    #[arg(long, group = "field")]
    since: Option<jiff::civil::Date>,
    // RecordingYear(Option<Operator>, u16),
    // RecordingMonth(Option<Operator>, u8),
    // Temperature(Option<Operator>, f64),
    #[arg(long, group = "field")]
    automated: Option<bool>,
    #[arg(short, long, group = "field")]
    device: Option<String>,
    #[arg(short, long, group = "field")]
    microphone: Option<String>,
    // SampleRate(Option<Operator>, u64),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let xcservice = xenocanto::Client::with_key(&std::env::var("XC_API_KEY")?);
    let mut query = Query::default();
    if let Some(val) = args.genus {
        query = query.field(SearchField::Genus(val));
    }
    if let Some(val) = args.species {
        query = query.field(SearchField::Species(val));
    }
    if let Some(val) = args.subspecies {
        query = query.field(SearchField::Subspecies(val));
    }
    if let Some(val) = args.recordist {
        query = query.field(SearchField::Recordist(val));
    }
    if let Some(val) = args.country {
        query = query.field(SearchField::Country(val));
    }
    if let Some(val) = args.location {
        query = query.field(SearchField::Location(val));
    }
    if let Some(val) = args.remarks {
        query = query.field(SearchField::Remarks(val));
    }
    if let Some(val) = args.seen {
        query = query.field(SearchField::Seen(val));
    }
    if let Some(val) = args.playback {
        query = query.field(SearchField::Playback(val));
    }
    if let Some(val) = args.id {
        query = query.field(SearchField::RecordingId(val));
    }
    if let Some(val) = args.since {
        query = query.field(SearchField::Since(val));
    }
    if let Some(val) = args.automated {
        query = query.field(SearchField::Automated(val));
    }
    if let Some(val) = args.device {
        query = query.field(SearchField::Device(val));
    }
    if let Some(val) = args.microphone {
        query = query.field(SearchField::Microphone(val));
    }

    let n = xcservice.download_all(query, args.out_dir).await?;
    println!("downloaded {n} files",);
    Ok(())
}
