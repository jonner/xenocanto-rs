use xenocanto::search::SearchTerm;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let xcservice = xenocanto::Service::with_key(&std::env::var("XC_API_KEY")?);
    let out = xcservice
        .query()
        .add_term(SearchTerm::RecordingId(254462))
        .send()
        .await?;
    dbg!(&out);
    Ok(())
}
