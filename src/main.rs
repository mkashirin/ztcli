use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    match ztcli::cli().await {
        Ok(_) => {}
        Err(err) => {
            eprintln!("ZTCLI returned an error: {err}");
            std::process::exit(1);
        }
    };
    Ok(())
}
