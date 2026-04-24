use std::io::Read;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "example-output".into());
    let mut f = std::fs::File::open(file)?;
    let mut response_text = String::new();
    f.read_to_string(&mut response_text)?;
    let response: xenocanto::recording::RecordingSet = serde_json::from_str(&response_text)?;
    dbg!(&response);
    Ok(())
}
