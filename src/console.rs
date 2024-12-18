use crate::{BaseClient, ClientConfig, Result, threats::Threats};

pub struct ConsoleClient {
    base: BaseClient,
    pub threats: Threats,
}

impl ConsoleClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base = BaseClient::new(config)?;
        Ok(Self {
            threats: Threats::new(base.clone()),
            base,
        })
    }
}
