use foxglove::WebSocketServer;

pub fn get_new_server(port: String) -> WebSocketServer {
    WebSocketServer::new().start()
}
