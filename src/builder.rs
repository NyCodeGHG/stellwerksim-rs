use std::net::SocketAddr;
use crate::{Error, Plugin, PluginDetails};

#[derive(Debug, Default)]
pub struct PluginBuilder {
    name: Option<String>,
    author: Option<String>,
    version: Option<String>,
    description: Option<String>,
    host: Option<SocketAddr>,
}

impl PluginBuilder {
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn author<S: Into<String>>(mut self, author: S) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn host<S: Into<SocketAddr>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    pub async fn connect(self) -> Result<Plugin, Error> {
        Plugin::connect(PluginDetails {
            name: self.name.unwrap_or_else(|| "stellwerksim-rs Plugin".into()),
            author: self
                .author
                .unwrap_or_else(|| "stellwerksim-rs Author".into()),
            version: self
                .version
                .unwrap_or_else(|| env!("CARGO_PKG_VERSION").into()),
            description: self
                .description
                .unwrap_or_else(|| "A stellwerksim-rs plugin".into()),
            host: self
                .host
                .unwrap_or_else(|| "127.0.0.1:3691".parse().unwrap()),
        })
        .await
    }
}


