use crate::{Error, Plugin, PluginDetails};
use std::net::SocketAddr;

/// Builder for constructing a [Plugin].
#[derive(Debug, Default)]
pub struct PluginBuilder<'a> {
    name: Option<&'a str>,
    author: Option<&'a str>,
    version: Option<&'a str>,
    description: Option<&'a str>,
    host: Option<SocketAddr>,
}

impl<'a> PluginBuilder<'a> {
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn author(mut self, author: &'a str) -> Self {
        self.author = Some(author);
        self
    }

    pub fn version(mut self, version: &'a str) -> Self {
        self.version = Some(version);
        self
    }

    pub fn description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn host(mut self, host: SocketAddr) -> Self {
        self.host = Some(host);
        self
    }

    pub async fn connect(self) -> Result<Plugin, Error> {
        Plugin::connect(PluginDetails {
            name: self.name.unwrap_or("stellwerksim-rs Plugin"),
            author: self.author.unwrap_or("stellwerksim-rs Author"),
            version: self
                .version
                .unwrap_or(concat!("stellwerksim-rs/", env!("CARGO_PKG_VERSION"))),
            description: self
                .description
                .unwrap_or("A stellwerksim-rs plugin"),
            host: self
                .host
                .unwrap_or_else(|| "127.0.0.1:3691".parse().unwrap()),
        })
        .await
    }
}
