use stellwerksim::Plugin;

#[tokio::main()]
async fn main() {
    let plugin = Plugin::builder()
        .name("Example Plugin")
        .version("1.0.0")
        .author("uwumarie")
        .connect()
        .await
        .unwrap();
    let time = plugin.simulator_time().await.unwrap();
    println!("{time:?}");
    let info = plugin.system_info().await.unwrap();
    println!("{info:#?}");
}
