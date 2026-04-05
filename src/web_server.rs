use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::Deserialize;
use crate::http_header_analyzer::analyze_headers;

/// Web sunucusunu başlatır ve `/` ile `/analyze` endpoint'lerini tanımlar.
pub async fn start_server() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/analyze", get(analyze_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🌐 Panel açık: http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

/// Ana sayfa handler'ı — HTML paneli döndürür.
async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

/// URL parametresi
#[derive(Deserialize)]
struct AnalyzeParams {
    url: String,
}

/// Analiz sonucunu JSON olarak döndürür.
async fn analyze_handler(
    Query(params): Query<AnalyzeParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    match analyze_headers(&params.url).await {
        Ok(report) => Ok(Json(serde_json::to_value(report).unwrap())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e)),
    }
}
