mod dbc;
mod flatbuf;
mod socketcan_utils;

pub fn test_can() {
    socketcan_utils::test_can().unwrap()
}
