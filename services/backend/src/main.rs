mod auth;
mod db;

use auth::Auth;
use axum::{
    Extension, Router,
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::get,
    serve,
};
use axum_extra::extract::CookieJar;
use db::init_db;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: sqlx::Pool<sqlx::Postgres>,
    auth: Arc<Auth>,
}

#[derive(Deserialize)]
struct QueryParams {
    limit: Option<i64>,
    search: Option<String>,
    kind: Option<String>,
    tag: Option<String>,
    user: Option<String>,
    app: Option<String>,
    random: Option<bool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        db: init_db().await?,
        auth: Arc::new(Auth::new()?),
    };

    let app = Router::new()
        .route("/api/videos", get(videos))
        .route("/video/get/{filename}", get(get_video))
        .layer(Extension(Arc::new(state)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8007));

    let listener = TcpListener::bind(addr).await?;

    serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn videos(
    jar: CookieJar,
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let email = match state.auth.validate(&jar) {
        Ok(email) => email,
        Err(_) => {
            return (StatusCode::OK, Json(Vec::<serde_json::Value>::new())).into_response();
        }
    };

    let row: (serde_json::Value,) = match sqlx::query_as(
        "SELECT video.get_user_videos($1, $2, $3, $4, $5::uuid, $6::uuid, $7::uuid, $8);",
    )
    .bind(&email)
    .bind(&params.limit)
    .bind(&params.search)
    .bind(&params.kind)
    .bind(&params.tag)
    .bind(&params.user)
    .bind(&params.app)
    .bind(&params.random)
    .fetch_one(&state.db)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response();
        }
    };

    (StatusCode::OK, Json(row.0)).into_response()
}

async fn get_video(
    jar: CookieJar,
    Extension(state): Extension<Arc<AppState>>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    let email = match state.auth.force_validate(&jar) {
        Ok(email) => email,
        Err(resp) => return resp,
    };

    let uuid_str = filename.strip_suffix(".mp4").unwrap_or(&filename);

    let has_access: (bool,) = match sqlx::query_as(
        "SELECT EXISTS (
            SELECT 1
            FROM Video.Userpins up
            JOIN Public.Users u ON u.id = up.uid
            WHERE u.email = $1 AND up.vid = $2::uuid
        )",
    )
    .bind(&email)
    .bind(uuid_str)
    .fetch_one(&state.db)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
        }
    };

    if !has_access.0 {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }

    let internal_path = format!("/videos/{}", filename);

    let mut response = Response::new(Body::empty());
    response
        .headers_mut()
        .insert("X-Accel-Redirect", internal_path.parse().unwrap());
    response
        .headers_mut()
        .insert("Content-Type", "video/mp4".parse().unwrap());

    response
}
