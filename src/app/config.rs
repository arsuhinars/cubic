use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub resolution: [u32; 2],
    pub max_frame_rate: f64,
}

impl AppConfig {
    pub fn load_from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;

        Ok(toml::from_str(&buf)?)
    }
}
