mod flatbuf;
mod mrow;
mod socketcan_utils;

pub fn test_can() {
    socketcan_utils::test_can().unwrap()
}

pub fn test_dbc() {
    mrow::mrow()
}
