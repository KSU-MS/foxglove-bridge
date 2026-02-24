mod commands;
mod foxglove_utils;
mod random_utils;
mod sources;

use foxglove::{
    schemas::{log::Level, Log, Timestamp},
    Channel, Context,
};
use foxglove_utils::FoxgloveRuntime;
use random_utils::pretty_print_system_time;

fn main() {
    let mut can_test = FoxgloveRuntime::start(
        Context::new(),
        pretty_print_system_time() + "-CAN.mcap",
        8765,
    )
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));

    let test_topic: Channel<Log> = can_test.ctx.channel_builder("/mrow").build();

    std::thread::sleep(std::time::Duration::from_secs(5));

    test_topic.log(&Log {
        level: Level::Info.into(),
        timestamp: Some(Timestamp::now()),
        message: "Morwoa :3".to_string(),
        ..Default::default()
    });

    std::thread::sleep(std::time::Duration::from_secs(5));

    test_topic.log(&Log {
        level: Level::Info.into(),
        timestamp: Some(Timestamp::now()),
        message: "Mrow twua".to_string(),
        ..Default::default()
    });

    std::thread::sleep(std::time::Duration::from_secs(2));

    can_test.stop();
}
