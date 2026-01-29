mod mcap_sink;
mod websocket_sink;

use mcap_sink::{close_mcap_writer, get_new_mcap_writer};

pub fn start_foxglove_log() {
    let mcap_writer = get_new_mcap_writer("mrow".to_string());
}

pub fn stop_foxglove_log() {}
