use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Service {
    name: String,
    url: String,
}

type Registry = Arc<DashMap<String, Service>>;

#[tokio::main]
async fn main() {
    let registry: Registry = Arc::new(DashMap::new());

    let app = Router::new()
        .route("/register", post(register_service))
        .route("/discover/:name", get(discover_service))
        .with_state(registry.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("🕸️ Service Mesh running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(registry))
        .await
        .unwrap();
}

async fn register_service(
    axum::Json(service): axum::Json<Service>,
    state: axum::extract::State<Registry>,
) -> Json<serde_json::Value> {
    state.insert(service.name.clone(), service.clone());
    Json(json!({ "status": "registered", "service": service }))
}

async fn discover_service(
    Path(name): Path<String>,
    state: axum::extract::State<Registry>,
) -> Json<serde_json::Value> {
    if let Some(svc) = state.get(&name) {
        Json(json!({ "status": "found", "service": svc.clone() }))
    } else {
        Json(json!({ "status": "not found", "service": name }))
    }
}
