use crate::{WebSocketReceiver, WebSocketSender};
use async_std::sync::Arc;
use futures::{SinkExt, StreamExt};
use tide_websockets::{WebSocketConnection, WebSocket};

pub fn split(
    stream: tide_websockets::WebSocketConnection,
) -> (WebSocketSender, WebSocketReceiver) {
    let (tx, rx) = async_std::channel::bounded(32);
    let (sink, stream) = stream.split();

    async_std::task::spawn(async move {
        let mut sink = sink;
        let mut stream = stream;

        while let Some(msg) = stream.next().await {
            match msg {
                Ok(msg) => {
                    if let Err(err) = sink.send(msg).await {
                        eprintln!("Error sending WebSocket message: {:?}", err);
                    }
                }
                Err(err) => eprintln!("Error receiving WebSocket message: {:?}", err),
            }
        }
    });

    (tx, rx)
}

pub async fn handle_messages(
    sender: WebSocketSender,
    mut receiver: WebSocketReceiver,
    connections: &mut Vec<WebSocketSender>,
) {
    while let Some(msg) = receiver.next().await {
        for conn in connections.iter() {
            if Arc::ptr_eq(&sender, conn) {
                continue;
            }

            if let Err(err) = conn.send(msg.clone()).await {
                eprintln!("Error broadcasting message: {:?}", err);
            }
        }
    }
}
