mod flatbuf;
mod socketcan_utils;
use dbc_rs::Dbc;

pub fn test_can() {
    let dbc_path =
        std::path::PathBuf::from(std::env::var_os("DBC_PATH").expect("DBC_PATH not set"))
            .join("car.dbc");

    let dbc = Dbc::from_file(&dbc_path).expect("Failed to load car.dbc");

    socketcan_utils::test_can(dbc).unwrap()
}
