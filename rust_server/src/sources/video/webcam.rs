// Basically, the Pi could probably not handle the load of transcoding MJPEG to h264, and foxglove was being
// bloody awful with the specific configuration of h264 with ffmpeg commands and the such, the
// h264_webcam_stream example does work but it drops frames pretty offten for unknown reasons. This
// ontop of all the very suprisingly good work from gopro @ https://gopro.github.io/OpenGoPro/
// means that the gopro implementation of this could look increadibly diffrent, but either way for
// now I should focus on the actual task at hand instead of playing around with this...

use std::time::{Duration, Instant};

use foxglove::{bytes::Bytes, schemas::CompressedVideo, LazyChannel, McapWriter, WebSocketServer};

static VIDEO: LazyChannel<CompressedVideo> = LazyChannel::new("/video");

pub fn test_cam() {
    // Create an MCAP file
    let _mcap_file = McapWriter::new()
        .create_new_buffered_file("./mrow.mcap")
        .unwrap();

    // Start WebSocket server
    WebSocketServer::new()
        .start_blocking()
        .expect("Server failed to start");

    // Frame timing
    let max_fps = 30;
    let frame_interval = Duration::from_secs_f32(1.0 / max_fps as f32);
    let mut last_frame = Instant::now();

    // Cam setup
    let device_path = std::path::Path::new("/dev/video0");
    let mut device = h264_webcam_stream::get_device(device_path).unwrap();
    let mut stream = h264_webcam_stream::stream(&mut device, max_fps).unwrap();

    for _ in 0..120 {
        // Get cam bytes
        let (h264_bytes, _) = stream.next(false).unwrap();
        let bytes = Bytes::from(h264_bytes);

        // Log
        VIDEO.log(&CompressedVideo {
            data: bytes,
            format: "h264".to_string(),
            ..Default::default()
        });

        // Throttle to max_fps
        let elapsed = last_frame.elapsed();
        if elapsed < frame_interval {
            std::thread::sleep(frame_interval - elapsed);
        }
        last_frame = Instant::now();
    }
}
