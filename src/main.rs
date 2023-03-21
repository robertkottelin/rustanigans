use async_std::sync::{Arc, Mutex};
use tide::sessions::SessionMiddleware;
use tide::{Request, Response, StatusCode};
use tide_websockets::{WebSocket, WebSocketConnection};
use rusqlite::Connection;
use std::collections::HashMap;

mod api;
mod db;
mod models;
mod ws;

pub type WebSocketSender = async_std::channel::Sender<String>;
pub type WebSocketReceiver = async_std::channel::Receiver<String>;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub connections: Arc<Mutex<HashMap<String, WebSocketSender>>>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let db = db::connect().await?;

    let state = AppState {
        db: Arc<async_std::sync::Mutex<rusqlite::Connection>>, // Replace with your actual DB instance
        connections: Arc::new(Mutex::new(HashMap::new())),
    };

    let mut app = tide::with_state(state);

    app.with(SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        b"this_is_a_random_secret",
    ));

    app.at("/ws").get(WebSocket::new(|req, ws_stream| handle_websocket(req, ws_stream)));
    app.at("/register").post(api::register_user);
    app.at("/login").post(api::login_user);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn handle_websocket(
    request: Request<AppState>,
    stream: WebSocketConnection,
) -> tide::Result<()> {
    let (sender, receiver) = ws::split(stream);
    let mut connections = request.state().connections.lock().await;

    connections.push(sender.clone());

    ws::handle_messages(sender, receiver, &mut connections).await
}
