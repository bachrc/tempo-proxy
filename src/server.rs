use anyhow::Context;
use axum::{Json, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_rapidoc::RapiDoc;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tempo Proxy API",
        description = "Un proxy fictif pour Tempo",
        version = "0.1.0"
    )
)]
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

pub async fn run(interface: &str) -> Result<(), anyhow::Error> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(bonjour))
        .split_for_parts();

    let router = router
        .route("/openapi.json", get(move || async { Json(api) }))
        .merge(RapiDoc::new("/openapi.json").path("/rapidoc"));

    let addr: SocketAddr = interface
        .parse()
        .context("parcours de l'interface renseignée")?;

    tracing::info!("Le serveur écoute sur http://{}", addr);
    tracing::info!("Spécification OpenAPI disponible sur http://{}/openapi.json", addr);
    tracing::info!("Documentation API disponible sur http://{}/rapidoc", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router)
        .await
        .context("écoute du serveur")?;

    Ok(())
}
