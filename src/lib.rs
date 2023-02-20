use std::net::SocketAddr;

pub use builder::PluginBuilder;
use serde::Deserialize;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::protocol::Status;

mod builder;
pub mod protocol;

struct PluginDetails {
    name: String,
    author: String,
    version: String,
    description: String,
    host: SocketAddr,
}

#[derive(Debug)]
pub struct Plugin {
    stream: BufReader<TcpStream>,
}

type Error = Box<dyn std::error::Error>;

impl Plugin {
    pub fn builder() -> PluginBuilder {
        PluginBuilder::default()
    }

    pub(crate) async fn connect(details: PluginDetails) -> Result<Self, Error> {
        let stream = BufReader::new(TcpStream::connect(details.host).await?);
        let mut plugin = Plugin { stream };
        let status = plugin.read_message::<Status>().await?;
        assert_eq!(
            300, status.code,
            "Received an invalid status code from StellwerkSim: {}",
            status.code
        );

        plugin.register(&details).await?;
        let status = plugin.read_message::<Status>().await?;
        assert_eq!(
            220, status.code,
            "Received an invalid status code from StellwerkSim: {}",
            status.code
        );
        Ok(plugin)
    }

    async fn register(
        &mut self,
        PluginDetails {
            ref name,
            ref author,
            ref version,
            ref description,
            ..
        }: &PluginDetails,
    ) -> Result<(), Error> {
        self.stream
            .write_all(format!("<register name='{name}' autor='{author}' version='{version}' protokoll='1' text='{description}' />\n").as_bytes())
            .await?;
        Ok(())
    }

    async fn read_message<'a, T: Deserialize<'a>>(&mut self) -> Result<T, Error> {
        let mut buf = String::new();
        self.stream.read_line(&mut buf).await?;
        Ok(serde_xml_rs::from_str(&buf)?)
    }
}
