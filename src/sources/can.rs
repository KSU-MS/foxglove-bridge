mod can_2_fb;
mod socketcan_utils;
use dbc_rs::Dbc;

pub fn test_can() {
    let dbc_path =
        std::path::PathBuf::from(std::env::var_os("DBC_PATH").expect("DBC_PATH not set"))
            .join("car.dbc");

    let dbc = Dbc::from_file(&dbc_path).expect("Failed to load car.dbc");

    // for message in dbc.messages().iter() {
    //     println!("{:?}", message.name())
    // }

    let ex_buf = vec![235, 25, 176, 30, 235, 177, 215, 30];

    println!("{:?}", dbc.decode(0x0A3, &ex_buf, false).unwrap());
    println!("{:?}", dbc.messages().find_by_id(0x0A3).unwrap().name());

    // socketcan_utils::test_can(dbc).unwrap()
}
