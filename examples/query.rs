use clap::Parser;

use xenocanto::SearchField;

#[derive(clap::Parser)]
#[command(group(
    clap::ArgGroup::new("field")
        .required(true)
        .multiple(true),
))]
struct Args {
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
    let mut query = xcservice.build_query();
    if let Some(val) = args.page_size {
        query = query.page_size(val);
    }

    if let Some(val) = args.genus {
        query = query.and(SearchField::Genus(val));
    }
    if let Some(val) = args.species {
        query = query.and(SearchField::Species(val));
    }
    if let Some(val) = args.subspecies {
        query = query.and(SearchField::Subspecies(val));
    }
    if let Some(val) = args.recordist {
        query = query.and(SearchField::Recordist(val));
    }
    if let Some(val) = args.country {
        query = query.and(SearchField::Country(val));
    }
    if let Some(val) = args.location {
        query = query.and(SearchField::Location(val));
    }
    if let Some(val) = args.remarks {
        query = query.and(SearchField::Remarks(val));
    }
    if let Some(val) = args.seen {
        query = query.and(SearchField::Seen(val));
    }
    if let Some(val) = args.playback {
        query = query.and(SearchField::Playback(val));
    }
    if let Some(val) = args.id {
        query = query.and(SearchField::RecordingId(val));
    }
    if let Some(val) = args.since {
        query = query.and(SearchField::Since(val));
    }
    if let Some(val) = args.automated {
        query = query.and(SearchField::Automated(val));
    }
    if let Some(val) = args.device {
        query = query.and(SearchField::Device(val));
    }
    if let Some(val) = args.microphone {
        query = query.and(SearchField::Microphone(val));
    }
    let out = query.fetch_page(1).await?;
    println!(
        "Got page {} of {} ({} total recordings)",
        out.page, out.total_pages, out.total_recordings
    );
    for rec in out.recordings {
        println!(
            "XC{} {} {} | {} | {} | {}",
            rec.id, rec.genus, rec.species, rec.info_uri, rec.recordist, rec.country
        )
    }
    Ok(())
}
