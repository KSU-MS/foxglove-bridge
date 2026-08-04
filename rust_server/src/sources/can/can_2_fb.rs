#[allow(warnings)]
mod gen_flatbuffers {
    include!(concat!(env!("OUT_DIR"), "/flatbuffers/mod.rs"));
}

pub fn can2fb(data: Vec<u8>) {}
