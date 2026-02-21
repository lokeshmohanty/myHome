mod app;
mod db;
mod modules;

// Desktop entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()?;
    Ok(())
}
