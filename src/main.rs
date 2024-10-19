use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    ztcli::cli().await?;
    Ok(())
}
