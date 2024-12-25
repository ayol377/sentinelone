use crate::{BaseClient, ClientConfig, Result, endpoints::{threats::Threats, agents::Agents, sites::Sites}};

pub struct ConsoleClient {
    pub base: BaseClient,
    pub threats: Threats,
    pub agents: Agents,
    pub sites: Sites,
}

impl ConsoleClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base = BaseClient::new(config)?;
        Ok(Self {
            threats: Threats::new(base.clone()),
            agents: Agents::new(base.clone()),
            sites: Sites::new(base.clone()),
            base,
        })
    }
}
