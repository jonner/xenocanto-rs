use std::io::Read;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut f = std::fs::File::open("example-output")?;
    let mut response_text = String::new();
    f.read_to_string(&mut response_text)?;
    let response: xenocanto::recording::RecordingSet = serde_json::from_str(&response_text)?;
    dbg!(&response);
    Ok(())
}
