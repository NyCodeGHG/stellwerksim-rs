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

    let platform_list = plugin.platform_list().await.unwrap();
    println!("{platform_list:#?}");

    let train_list = plugin.train_list().await.unwrap();
    println!("{train_list:#?}");

    let train_details = plugin
        .train_details(&train_list.get(0).unwrap().id)
        .await
        .unwrap();
    println!("{train_details:#?}");

    let train_timetable = plugin
        .train_timetable(&train_list.get(0).unwrap().id)
        .await
        .unwrap();
    println!("{train_timetable:#?}");

    let ways = plugin.ways().await.unwrap();
    println!("{ways:#?}");
}
