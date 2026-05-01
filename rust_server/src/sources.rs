mod audio;
mod can;
mod video;

use crate::random_utils::{parse_device_file, LoggerType};

pub fn start_logger() {}

pub fn test_cam() {
    video::test_cam();
}

pub fn test_can() {
    can::test_can();
}
