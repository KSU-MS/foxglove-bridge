use foxglove::{FoxgloveError, LazyContext, McapWriterHandle, WebSocketServerHandle};
use std::{fs::File, io::BufWriter, path::Path};

pub struct FoxgloveRuntime<'a> {
    ctx: &'a LazyContext,
    file: Option<McapWriterHandle<BufWriter<File>>>,
    socket: Option<WebSocketServerHandle>,
}

impl<'a> FoxgloveRuntime<'a> {
    pub fn start<P: AsRef<Path>>(
        ctx: &'a LazyContext,
        file: P,
        port: u16,
    ) -> Result<Self, FoxgloveError> {
        let mcap = ctx.mcap_writer().create_new_buffered_file(file)?;

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

impl Drop for FoxgloveRuntime<'_> {
    fn drop(&mut self) {
        self.stop();
    }
}
