mod audio;
mod can;
mod video;

pub fn test_cam() {
    video::test_cam();
}

pub fn test_can() {
    can::test_can();
}

pub fn test_dbc() {
    can::test_dbc();
}
