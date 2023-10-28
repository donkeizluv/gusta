use anyhow::Result;
use config::*;
use tui::AppTui;

mod api_provider;
mod assets;
mod client;
mod config;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::read_env()?;
    let mut app = AppTui::new(conf)?;
    app.start().await?;

    Ok(())
}
