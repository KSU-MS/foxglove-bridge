use foxglove::{FoxgloveError, McapWriter, McapWriterHandle};
use std::fs::File;
use std::io::BufWriter;

pub fn get_new_mcap_writer(file_path: String) -> McapWriterHandle<BufWriter<File>> {
    McapWriter::new()
        .create_new_buffered_file(file_path)
        .expect("Failed to start mcap writer")
}

pub fn close_mcap_writer(writer: McapWriterHandle<BufWriter<File>>) {
    match writer.close() {
        Ok(_) => {
            println!("Mcap closed.");
        }

        Err(FoxgloveError::SinkClosed) => {
            println!("MCAP already closed.");
        }

        Err(FoxgloveError::McapError(e)) => {
            eprintln!("MCAP error while closing: {e}");
        }

        Err(e) => {
            eprintln!("Failed to close mcap file: {e:?}");
        }
    }
}
