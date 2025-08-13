use anyhow::Context;
use axum::{Json, http::StatusCode, response::IntoResponse, routing::get, http::header, response::Response, body::Body};
use chrono::Local;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;

use crate::{cache, tempo_service};
use include_dir::{include_dir, Dir};

static WEB_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../web/build");

#[derive(Debug, Serialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TempoStatus {
    TempoBleu,
    TempoRouge,
    TempoBlanc,
    NonDefini,
}

#[derive(Debug, Serialize, ToSchema)]
struct CalendarResponse {
    date: String,
    statut: TempoStatus,
}

#[derive(Debug, Serialize, ToSchema)]
struct CalendarFullResponse {
    today: CalendarResponse,
    tomorrow: CalendarResponse,
}

#[derive(Debug, Serialize, ToSchema)]
struct ErrorResponse {
    message: String,
}

#[derive(OpenApi)]
#[openapi(info(
    title = "Tempo Proxy API",
    description = "Un proxy fictif pour Tempo",
    version = "0.1.0"
))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Message de test", body = String)
    )
)]
async fn bonjour() -> &'static str {
    "pouet"
}

#[utoipa::path(
    get,
    path = "/calendar/today",
    responses(
        (status = 200, description = "Calendrier du jour", body = CalendarResponse),
        (status = 400, description = "Erreur lors de l'appel à l'API EDF", body = ErrorResponse)
    )
)]
async fn calendar_today(
    axum::extract::State(cache): axum::extract::State<cache::TempoCache>
) -> impl IntoResponse {
    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    match tempo_service::get_tempo_status_for_date(&cache, today).await {
        Ok(status) => (
            StatusCode::OK,
            Json(CalendarResponse {
                date: today_str,
                statut: status,
            }),
        )
            .into_response(),
        Err(error_message) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error_message,
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/calendar",
    responses(
        (status = 200, description = "Calendrier aujourd'hui et demain", body = CalendarFullResponse),
        (status = 400, description = "Erreur lors de l'appel à l'API EDF", body = ErrorResponse)
    )
)]
async fn calendar_full(
    axum::extract::State(cache): axum::extract::State<cache::TempoCache>
) -> impl IntoResponse {
    let today = Local::now();
    let today_str = today.format("%Y-%m-%d").to_string();
    
    let tomorrow = today + chrono::Duration::days(1);
    let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();

    // Récupérer les statuts pour aujourd'hui et demain en une seule opération
    match tempo_service::get_tempo_status_for_today_and_tomorrow(&cache).await {
        Ok((today_status, tomorrow_status)) => (
            StatusCode::OK,
            Json(CalendarFullResponse {
                today: CalendarResponse {
                    date: today_str,
                    statut: today_status,
                },
                tomorrow: CalendarResponse {
                    date: tomorrow_str,
                    statut: tomorrow_status,
                },
            }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error,
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/calendar/tomorrow",
    responses(
        (status = 200, description = "Calendrier de demain", body = CalendarResponse),
        (status = 400, description = "Erreur lors de l'appel à l'API EDF", body = ErrorResponse)
    )
)]
async fn calendar_tomorrow(
    axum::extract::State(cache): axum::extract::State<cache::TempoCache>
) -> impl IntoResponse {
    let tomorrow = Local::now().date_naive() + chrono::Duration::days(1);
    let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();

    match tempo_service::get_tempo_status_for_date(&cache, tomorrow).await {
        Ok(status) => (
            StatusCode::OK,
            Json(CalendarResponse {
                date: tomorrow_str,
                statut: status,
            }),
        )
            .into_response(),
        Err(error_message) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: error_message,
            }),
        )
            .into_response(),
    }
}

fn get_mime_type(path: &str) -> &'static str {
    match std::path::Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

async fn serve_static(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match WEB_DIR.get_file(path) {
        Some(file) => Response::builder()
            .header(header::CONTENT_TYPE, get_mime_type(path))
            .body(Body::from(file.contents()))
            .unwrap(),
        None => {
            // Pour les routes SPA, fallback vers index.html
            match WEB_DIR.get_file("index.html") {
                Some(file) => Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(file.contents()))
                    .unwrap(),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not found"))
                    .unwrap(),
            }
        }
    }
}

pub async fn run(interface: &str) -> Result<(), anyhow::Error> {
    let cache = cache::create_cache();
    
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(bonjour))
        .routes(routes!(calendar_full))
        .routes(routes!(calendar_today))
        .routes(routes!(calendar_tomorrow))
        .split_for_parts();

    let router = router
        .with_state(cache)
        .route("/openapi.json", get(move || async { Json(api) }))
        .merge(RapiDoc::new("/openapi.json").path("/rapidoc"))
        .fallback(serve_static);

    let addr: SocketAddr = interface
        .parse()
        .context("parcours de l'interface renseignée")?;

    tracing::info!("Le serveur écoute sur http://{}", addr);
    tracing::info!(
        "Spécification OpenAPI disponible sur http://{}/openapi.json",
        addr
    );
    tracing::info!("Documentation API disponible sur http://{}/rapidoc", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router)
        .await
        .context("écoute du serveur")?;

    Ok(())
}
