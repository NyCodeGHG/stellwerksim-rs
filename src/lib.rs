//! # stellwerksim-rs
//! Rust SDK for writing asynchronous [StellwerkSim](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:schnittstelle) plugins using [tokio](https://crates.io/crates/tokio).
//!
//! # Implemented Request Types
//! Note: All request types except events are implemented. Feel free to
//! [contribute](https://github.com/NyCodeGHG/stellwerksim-rs).
//!
//! * [`register`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#register) - Automatically done by [Plugin]'s API.
//! * [`simzeit`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#simzeit) - See [Plugin::simulator_time].
//! Requires the `simulator-time` feature flag which is **Enabled by default!**
//! * [`bahnsteigliste`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#bahnsteigliste) - See [Plugin::platform_list].
//! * [`zugliste`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugliste) - See [Plugin::train_list].
//! * [`zugdetails`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugdetails) - See [Plugin::train_details].
//! * [`zugfahrplan`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugfahrplan) - See [Plugin::train_timetable].
//! Requires the `timetable` feature flag which is **Enabled by default!**
//! * [`wege`](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#wege) - See [Plugin::ways].
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
#![deny(
    unsafe_code,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_mut,
    unused_results,
    unused_lifetimes
)]

use crate::protocol::{
    Platform, PlatformListResponse, Status, SystemInfo, Train, TrainDetails, TrainListResponse,
    Ways,
};

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
    #[error("StellwerkSim returned an invalid status code: {}", .0.code)]
    InvalidResponse(Status),
}

impl Plugin {
    /// Creates a new [PluginBuilder].
    pub fn builder<'a>() -> PluginBuilder<'a> {
        PluginBuilder::default()
    }

    pub(crate) async fn connect(details: PluginDetails<'_>) -> Result<Self, Error> {
        let mut stream = BufReader::new(TcpStream::connect(details.host).await?);
        let status = read_message::<Status>(&mut stream, None).await?;
        if status.code != 300 {
            return Err(Error::InvalidResponse(status));
        }

        let stream = Mutex::new(stream);
        let plugin = Plugin { stream };
        let status = plugin.register(&details).await?;

        if status.code != 220 {
            return Err(Error::InvalidResponse(status));
        }
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
            .as_bytes(),
            None
        ).await
    }

    async fn send_request<'a, T: Deserialize<'a>>(
        &self,
        message: &[u8],
        ending_tag: Option<&str>,
    ) -> Result<T, Error> {
        let mut stream = self.stream.lock().await;
        stream.write_all(message).await?;
        stream.write_u8(b'\n').await?;
        stream.flush().await?;
        read_message(&mut stream, ending_tag).await
    }

    /// Retrievies the current in-game time. [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#simzeit)
    #[cfg(feature = "simulator-time")]
    pub async fn simulator_time(&self) -> Result<chrono::NaiveTime, Error> {
        use chrono::Utc;
        use protocol::simulator_time::SimulatorTimeResponse;

        let now = Utc::now();
        let response: SimulatorTimeResponse = self.send_request(b"<simzeit />", None).await?;
        let elapsed = now.signed_duration_since(Utc::now());
        Ok(response.time - elapsed / 2)
    }

    /// Reads information about the current system.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#anlageninfo)
    pub async fn system_info(&self) -> Result<SystemInfo, Error> {
        self.send_request(b"<anlageninfo />", None).await
    }

    /// Gets a full list of platforms.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#bahnsteigliste)
    pub async fn platform_list(&self) -> Result<Vec<Platform>, Error> {
        Ok(self
            .send_request::<PlatformListResponse>(b"<bahnsteigliste />", Some("</bahnsteigliste>"))
            .await?
            .platforms)
    }

    /// Gets a full list of trains.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugliste)
    pub async fn train_list(&self) -> Result<Vec<Train>, Error> {
        Ok(self
            .send_request::<TrainListResponse>(b"<zugliste />", Some("</zugliste>"))
            .await?
            .trains)
    }

    /// Gets the train details by a train id.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugdetails)
    pub async fn train_details(&self, train_id: &str) -> Result<TrainDetails, Error> {
        self.send_request(format!("<zugdetails zid='{train_id}' />").as_bytes(), None)
            .await
    }

    /// Gets the timeable of a train by it's train id.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#zugfahrplan)
    #[cfg(feature = "timetable")]
    pub async fn train_timetable(&self, train_id: &str) -> Result<protocol::TrainTimetable, Error> {
        self.send_request(
            format!("<zugfahrplan zid='{train_id}' />").as_bytes(),
            Some("</zugfahrplan>"),
        )
        .await
    }

    /// Gets a full list of shapes and connections of the track diagram.
    /// [Official docs](https://doku.stellwerksim.de/doku.php?id=stellwerksim:plugins:spezifikation#wege)
    pub async fn ways(&self) -> Result<Ways, Error> {
        self.send_request(b"<wege />", Some("</wege>")).await
    }
}

// ending_tag is required if the response has more than one line
async fn read_message<'a, T: Deserialize<'a>>(
    stream: &mut BufReader<TcpStream>,
    ending_tag: Option<&str>,
) -> Result<T, Error> {
    let mut buf = String::new();
    if let Some(ending_tag) = ending_tag {
        loop {
            let mut loop_buf = String::new();
            let _ = stream.read_line(&mut loop_buf).await?;
            buf += &loop_buf;
            if loop_buf.trim() == ending_tag {
                break;
            }
        }
    } else {
        let _ = stream.read_line(&mut buf).await?;
    }

    Ok(serde_xml_rs::from_str(&buf)?)
}
