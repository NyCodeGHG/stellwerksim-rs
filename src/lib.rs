use std::{error::Error, net::SocketAddr};

pub use builder::PluginBuilder;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};

mod builder;
pub mod protocol;

struct PluginDetails {
    name: String,
    author: String,
    version: String,
    description: String,
    host: SocketAddr,
}

pub struct Plugin {
    stream: BufReader<TcpStream>,
}

impl Plugin {
    pub fn builder() -> PluginBuilder {
        PluginBuilder::default()
    }

    pub(crate) async fn connect(details: PluginDetails) -> Result<Self, Box<dyn Error>> {
        let mut stream = BufReader::new(TcpStream::connect(details.host).await?);
        let mut buf = String::new();
        stream.read_line(&mut buf).await?;
        Ok(Plugin { stream })
    }
}
