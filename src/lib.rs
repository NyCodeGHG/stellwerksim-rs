use std::net::SocketAddr;

pub use builder::PluginBuilder;
use serde::Deserialize;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};

use crate::protocol::{SimulatorTime, Status, SystemInfo};

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
    stream: Mutex<BufReader<TcpStream>>,
}

type Error = Box<dyn std::error::Error>;

impl Plugin {
    pub fn builder() -> PluginBuilder {
        PluginBuilder::default()
    }

    pub(crate) async fn connect(details: PluginDetails) -> Result<Self, Error> {
        let mut stream = BufReader::new(TcpStream::connect(details.host).await?);
        let status = read_message::<Status>(&mut stream).await?;
        assert_eq!(
            300, status.code,
            "Received an invalid status code from StellwerkSim: {}",
            status.code
        );

        let stream = Mutex::new(stream);
        let plugin = Plugin { stream };
        let status = plugin.register(&details).await?;

        assert_eq!(
            220, status.code,
            "Received an invalid status code from StellwerkSim: {}",
            status.code
        );
        Ok(plugin)
    }

    async fn register(
        &self,
        PluginDetails {
            ref name,
            ref author,
            ref version,
            ref description,
            ..
        }: &PluginDetails,
    ) -> Result<Status, Error> {
        self.send_request(
            format!("<register name='{name}' autor='{author}' version='{version}' protokoll='1' text='{description}' />")
            .as_bytes()
        ).await
    }

    async fn send_request<'a, T: Deserialize<'a>>(&self, message: &[u8]) -> Result<T, Error> {
        let mut stream = self.stream.lock().await;
        stream.write_all(message).await?;
        stream.write_u8(b'\n').await?;
        stream.flush().await?;
        read_message(&mut stream).await
    }

    pub async fn simulator_time(&self) -> Result<SimulatorTime, Error> {
        self.send_request(b"<simzeit />").await
    }

    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        self.send_request(b"<anlageninfo />").await
    }
}

async fn read_message<'a, T: Deserialize<'a>>(
    stream: &mut BufReader<TcpStream>,
) -> Result<T, Error> {
    let mut buf = String::new();
    stream.read_line(&mut buf).await?;
    Ok(serde_xml_rs::from_str(&buf)?)
}
