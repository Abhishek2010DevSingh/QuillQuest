pub trait ConfigProvider {
    fn get_port(&self) -> anyhow::Result<u32> {
        Ok(8080)
    }
}
