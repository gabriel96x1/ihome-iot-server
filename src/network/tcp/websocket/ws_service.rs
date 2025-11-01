use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::Response;

pub async fn ws_communication(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_ws_socket)
}

async fn handle_ws_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("client sent str: {:?}", t);
                    if socket.send(Message::Text(format!("You sent: {}", t).into())).await.is_err() {
                        println!("client disconnected");
                        return;
                    }
                }
                Message::Binary(b) => {
                    println!("client sent binary: {:?}", b);
                    if socket.send(Message::Binary(b)).await.is_err() {
                        println!("client disconnected");
                        return;
                    }
                }
                Message::Ping(p) => {
                    println!("client sent ping: {:?}", p);
                }
                Message::Pong(p) => {
                    println!("client sent pong: {:?}", p);
                }
                Message::Close(c) => {
                    println!("client sent close: {:?}", c);
                    return;
                }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }

}