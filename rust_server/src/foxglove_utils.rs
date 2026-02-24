use foxglove::{Context, FoxgloveError, McapWriterHandle, WebSocketServerHandle};
use std::sync::Arc;
use std::{fs::File, io::BufWriter, path::Path};

pub struct FoxgloveRuntime {
    pub ctx: Arc<Context>,
    file: Option<McapWriterHandle<BufWriter<File>>>,
    socket: Option<WebSocketServerHandle>,
}

impl FoxgloveRuntime {
    pub fn start<P: AsRef<Path>>(
        ctx: Arc<Context>,
        file_path: P,
        port: u16,
    ) -> Result<Self, FoxgloveError> {
        let mcap = ctx.mcap_writer().create_new_buffered_file(file_path)?;

        let ws = ctx
            .websocket_server()
            .bind("127.0.0.1", port)
            .start_blocking()?;

        Ok(Self {
            ctx,
            file: Some(mcap),
            socket: Some(ws),
        })
    }

    pub fn stop(&mut self) {
        if let Some(mcap) = self.file.take() {
            let _ = mcap.close();
        }

        if let Some(ws) = self.socket.take() {
            ws.stop().wait_blocking();
        }
    }

    pub fn is_running(&self) -> bool {
        self.file.is_some() && self.socket.is_some()
    }
}

impl Drop for FoxgloveRuntime {
    fn drop(&mut self) {
        self.stop();
    }
}
