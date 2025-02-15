use std::env;

use anyhow::{Context, Result};

use super::interface::ConfigProvider;

pub struct EnvConfigProvider;

impl ConfigProvider for EnvConfigProvider {
    fn get_port(&self) -> Result<u32> {
        let port_str = env::var("PORT").context("'PORT' environment variable is not set")?;
        let port = port_str
            .parse()
            .context("Failed to parse 'PORT' as a valid u32")?;
        Ok(port)
    }
}
