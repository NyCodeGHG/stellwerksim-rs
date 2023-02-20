//! # stellwerksim-rs
//! Rust SDK for writing asynchronous [StellwerkSim](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:schnittstelle) plugins using [tokio](https://crates.io/tokio).
//!
//! # Implemented Request Types
//! Note: Not all request types are implemented yet. Feel free to
//! [contribute](https://github.com/NyCodeGHG/stellwerksim-rs).
//!
//! * [`register`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#register) - Automatically done by [Plugin]'s API.
//! * [`simzeit`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#simzeit) - See [Plugin::simulator_time].
//! Requires the `simulator-time` feature flag which is **Enabled by default!**
//!
//! # Example
//! Create plugin instance via the [Plugin::builder].
//! It will connect to StellwerkSim and register the plugin automatically.
//! ```no_run
//! use stellwerksim::{Error, Plugin};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let plugin = Plugin::builder()
//!         .name("My stellwerksim-rs Plugin")
//!         .author("Me")
//!         .version(env!("CARGO_PKG_VERSION")) // Embed version from Cargo.toml
//!         .description("My plugin built with stellwerksim-rs")
//!         .connect().await?;
//!     Ok(())
//! }
//! ```

use crate::protocol::{Status, SystemInfo};
use serde::Deserialize;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};

pub use builder::PluginBuilder;
mod builder;
/// StellwerkSim's xml based protocol.
pub mod protocol;

struct PluginDetails<'a> {
    name: &'a str,
    author: &'a str,
    version: &'a str,
    description: &'a str,
    host: SocketAddr,
}

/// A running StellwerkSim Plugin instance.
#[derive(Debug)]
pub struct Plugin {
    stream: Mutex<BufReader<TcpStream>>,
}

/// The errors which may occur when using a [Plugin].
#[derive(Debug, Error)]
pub enum Error {
    #[error("A network error occured: {0}")]
    Network(#[from] tokio::io::Error),
    #[error("Failed to parse xml: {0}")]
    Xml(#[from] serde_xml_rs::Error),
}

impl Plugin {
    /// Creates a new [PluginBuilder].
    pub fn builder<'a>() -> PluginBuilder<'a> {
        PluginBuilder::default()
    }

    pub(crate) async fn connect<'a>(details: PluginDetails<'a>) -> Result<Self, Error> {
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

    async fn register<'a>(
        &self,
        PluginDetails {
            ref name,
            ref author,
            ref version,
            ref description,
            ..
        }: &PluginDetails<'a>,
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

    /// Retrievies the current in-game time.
    #[cfg(feature = "simulator-time")]
    pub async fn simulator_time(&self) -> Result<chrono::NaiveTime, Error> {
        use chrono::Utc;
        use protocol::SimulatorTimeResponse;

        let now = Utc::now();
        let response: SimulatorTimeResponse = self.send_request(b"<simzeit />").await?;
        let elapsed = now.signed_duration_since(Utc::now());
        Ok(response.time - elapsed / 2)
    }

    /// Reads information about the current system.
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
