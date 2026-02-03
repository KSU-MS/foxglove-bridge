// use std::path::Path;
// use std::time::{Duration, Instant};

use nokhwa::{
    native_api_backend, nokhwa_initialize,
    pixel_format::RgbFormat,
    query,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

// use foxglove::{bytes::Bytes, schemas::CompressedVideo, LazyChannel, McapWriter, WebSocketServer};
//
// static VIDEO: LazyChannel<CompressedVideo> = LazyChannel::new("/video");

pub fn test_cam() {
    let backend = native_api_backend().unwrap();
    let devices = query(backend).unwrap();
    println!("There are {} available cameras.", devices.len());
    for device in devices {
        println!("{device}");
    }

    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(index, requested).unwrap();

    camera.open_stream().unwrap();
    for _ in 0..60 {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.resolution(), frame.source_frame_format());
    }

    // // Create an MCAP file
    // let _mcap_file = McapWriter::new()
    //     .create_new_buffered_file("./mrow.mcap")
    //     .unwrap();
    //
    // // Start WebSocket server
    // WebSocketServer::new()
    //     .start_blocking()
    //     .expect("Server failed to start");
    //
    // let device_path = Path::new("/dev/video0");
    // let max_fps = 60;
    //
    // let frame_interval = Duration::from_secs_f32(1.0 / max_fps as f32);
    //
    // let mut last_frame = Instant::now();
    //
    // for _ in 0..120 {
    //     let (h264_bytes, _) = stream.next(false).unwrap();
    //
    //     let bytes = Bytes::from(h264_bytes);
    //     VIDEO.log(&CompressedVideo {
    //         data: bytes,
    //         format: "h264".to_string(),
    //         ..Default::default()
    //     });
    //
    //     // Throttle to max_fps
    //     let elapsed = last_frame.elapsed();
    //     if elapsed < frame_interval {
    //         std::thread::sleep(frame_interval - elapsed);
    //     }
    //     last_frame = Instant::now();
    // }
}
