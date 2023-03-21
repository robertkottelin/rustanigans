use crate::db;
use crate::models::{LoginData, User};
use rusqlite::params;
use serde_json::json;
use tide::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

pub async fn register_user(mut request: Request<crate::AppState>) -> tide::Result {
    let user: User = request.body_json().await?;

    let conn = db::connect().await?;

    conn.execute(
        "INSERT INTO users (username, password    ) VALUES (?1, ?2)",
        params![user.username, user.password],
    )?;

    let res = json!({ "status": "success" });

    Ok(Response::new(StatusCode::Ok).set_body(json!(&res)))
}

pub async fn login_user(mut request: Request<crate::AppState>) -> tide::Result {
    let login_data: LoginData = request.body_json().await?;

    let conn = db::connect().await?;

    let user = conn.query_row(
        "SELECT id, username, password FROM users WHERE username = ?1",
        params![login_data.username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        },
    );

    match user {
        Ok(user) => {
            if user.password == login_data.password {
                let session = request.session_mut();
                session.insert("user_id", user.id)?;
                session.insert("username", user.username)?;

                let res = json!({ "status": "success" });

                Ok(Response::new(StatusCode::Ok).body(json!(&res)))

            } else {
                let res = json!({ "status": "error", "message": "Incorrect password" });

                Ok(Response::new(StatusCode::Ok).body(json!(&res)))
            }
        }
        Err(_) => {
            let res = json!({ "status": "error", "message": "User not found" });

            Ok(Response::new(StatusCode::Ok).set_body(json!(&res)))
        }
    }
}

