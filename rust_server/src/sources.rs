mod audio;
mod can;
mod video;

use crate::{foxglove_utils::FoxgloveRuntime, sources::can::test_can};

trait FoxgloveLoggerTopic {
    fn start_log(&self) -> Result<(), String>;
    fn stop_log(&self) -> Result<(), String>;
    fn register_messages(&mut self, foxglove_context: FoxgloveRuntime) -> Result<(), String>;
}

pub fn start_loggers(foxglove_context: FoxgloveRuntime) {
    test_can(foxglove_context);
}

// pub fn test_cam() {
//     video::test_cam();
// }
//
// pub fn test_can() {
//     can::test_can();
// }
