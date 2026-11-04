#[tokio::main]
async fn main() -> Result<(), openrazer::Error> {
    let client = openrazer::Client::connect().await?;
    println!(
        "OpenRazer {}: {:?}",
        client.daemon_version().await?,
        client.serials().await?
    );
    Ok(())
}
