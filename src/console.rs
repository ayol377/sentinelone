use crate::{endpoints::{activities::Activities, agents::Agents, sites::Sites, threats::Threats}, BaseClient, ClientConfig, Result};

pub struct ConsoleClient {
    pub base: BaseClient,
    pub threats: Threats,
    pub agents: Agents,
    pub sites: Sites,
    pub activities: Activities,
}

impl ConsoleClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base = BaseClient::new(config)?;
        Ok(Self {
            threats: Threats::new(base.clone()),
            agents: Agents::new(base.clone()),
            sites: Sites::new(base.clone()),
            activities: Activities::new(base.clone()),
            base,
        })
    }
}
