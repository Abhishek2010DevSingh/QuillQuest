use std::env;

use anyhow::{Context, Result};

use super::interface::ConfigProvider;

pub struct EnvConfigProvider {
    port: u32,
}

impl EnvConfigProvider {
    pub fn new() -> Result<Self> {
        let port_str = env::var("PORT").context("'PORT' environment variable is not set")?;
        let port = port_str
            .parse()
            .context("Failed to parse 'PORT' as a valid u32")?;
        Ok(Self { port })
    }
}

impl ConfigProvider for EnvConfigProvider {
    fn get_port(&self) -> Result<u32> {
        Ok(self.port)
    }
}
