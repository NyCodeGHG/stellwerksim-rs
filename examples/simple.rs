use stellwerksim::Plugin;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let plugin = Plugin::builder()
        .name("Example Plugin")
        .version("1.0.0")
        .author("uwumarie")
        .connect()
        .await
        .unwrap();
    dbg!(plugin);
}
