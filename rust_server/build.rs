use flatbuffers_build::BuilderOptions;
use std::env;

fn main() {
    let fbs_path = env::var("FBS_PATH").expect("FBS_PATH environment variable not set");

    BuilderOptions::new_with_files(&[&fbs_path])
        .compile()
        .expect("flatbuffer compilation failed");
}
