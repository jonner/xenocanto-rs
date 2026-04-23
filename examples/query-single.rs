use xenocanto::search::SearchTag;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let xcservice = xenocanto::Service::new(&std::env::var("XC_API_KEY")?);
    let out = xcservice
        .request([SearchTag::RecordingId(254462).into()])
        .await?;
    dbg!(&out);
    Ok(())
}
