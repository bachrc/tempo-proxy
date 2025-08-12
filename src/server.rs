use anyhow::Context;
use axum::{Json, http::StatusCode, response::IntoResponse, routing::get};
use chrono::Local;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;

use crate::{cache, tempo_service};

#[derive(Debug, Serialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TempoStatus {
    TempoBleu,
    TempoRouge,
    TempoBlanc,
}

#[derive(Debug, Serialize, ToSchema)]
struct CalendarTodayResponse {
    date: String,
    statut: TempoStatus,
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
        (status = 200, description = "Calendrier du jour", body = CalendarTodayResponse),
        (status = 400, description = "Erreur lors de l'appel à l'API EDF", body = ErrorResponse)
    )
)]
async fn calendar_today(
    axum::extract::State(cache): axum::extract::State<cache::TempoCache>
) -> impl IntoResponse {
    let today = Local::now();
    let today_str = today.format("%Y-%m-%d").to_string();

    match tempo_service::get_tempo_status_for_date(&cache, &today_str).await {
        Ok(status) => (
            StatusCode::OK,
            Json(CalendarTodayResponse {
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

pub async fn run(interface: &str) -> Result<(), anyhow::Error> {
    let cache = cache::create_cache();
    
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(bonjour))
        .routes(routes!(calendar_today))
        .split_for_parts();

    let router = router
        .with_state(cache)
        .route("/openapi.json", get(move || async { Json(api) }))
        .merge(RapiDoc::new("/openapi.json").path("/rapidoc"));

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
