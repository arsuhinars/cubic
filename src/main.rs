use anyhow::Context;
use app::{App, AppConfig};

mod app;
mod render;
mod utils;
mod world;

static CONFIG_PATH: &str = "config.toml";

#[pollster::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Box::new(
        AppConfig::load_from_file(CONFIG_PATH)
            .with_context(|| format!("Failed to load config file at {}", CONFIG_PATH))?,
    );
    let mut app = Box::new(App::new(cfg));
    app.run()
        .with_context(|| "Error occurred during app running")?;

    Ok(())
}
