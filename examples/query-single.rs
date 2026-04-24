use xenocanto::SearchField;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let xcservice = xenocanto::Client::with_key(&std::env::var("XC_API_KEY")?);
    let out = xcservice
        .build_query(SearchField::RecordingId(254462))
        .fetch_page(1)
        .await?;
    dbg!(&out);
    Ok(())
}
