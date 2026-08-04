mod commands;
mod foxglove_utils;
mod random_utils;
mod sources;

use foxglove::Context;
use foxglove_utils::FoxgloveRuntime;
use random_utils::pretty_print_system_time;

use sources::start_loggers;

fn main() {
    let global_fg: FoxgloveRuntime = FoxgloveRuntime::start(
        Context::new(),
        format!("{}.mcap", pretty_print_system_time()),
        8765,
    )
    .unwrap();

    start_loggers(global_fg);
}
