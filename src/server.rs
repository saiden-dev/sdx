use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::api::{
    ErrorResponse, ImageData, ImageGenerationRequest, ImageGenerationResponse, ModelInfo,
    ModelsResponse,
};
use crate::config::AppConfig;
use crate::sd_cli::GenerateArgs;

pub struct AppState {
    pub config: AppConfig,
    pub gpu_lock: Mutex<()>,
}

pub fn build_router(config: AppConfig) -> Router {
    let state = Arc::new(AppState {
        config,
        gpu_lock: Mutex::new(()),
    });

    Router::new()
        .route("/v1/models", get(list_models))
        .route("/v1/images/generations", post(generate_image))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn list_models(State(state): State<Arc<AppState>>) -> Json<ModelsResponse> {
    let models = state
        .config
        .models
        .keys()
        .map(|name| ModelInfo {
            id: name.clone(),
            object: "model",
            owned_by: "local",
        })
        .collect();

    Json(ModelsResponse {
        object: "list",
        data: models,
    })
}

async fn generate_image(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ImageGenerationRequest>,
) -> impl IntoResponse {
    // Resolve model name — use first configured model as default
    let model_name = req
        .model
        .as_deref()
        .or_else(|| state.config.models.keys().next().map(String::as_str));
    let Some(model_name) = model_name else {
        return (
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::to_value(ErrorResponse::new(
                    "no model specified and no models configured",
                ))
                .expect("error serializes"),
            ),
        );
    };

    let model_config = match state.config.resolve_model(model_name) {
        Ok(m) => m,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(
                    serde_json::to_value(ErrorResponse::not_found(format!(
                        "model '{model_name}' not found in config"
                    )))
                    .expect("error serializes"),
                ),
            );
        }
    };

    // Build generation args from model defaults + request overrides
    let mut args = GenerateArgs::from_model_config(&state.config.sd_cli_path, model_config);
    args.prompt = req.prompt.clone();

    if let Some(neg) = &req.negative_prompt {
        args.negative_prompt = Some(neg.clone());
    }
    if let Some((w, h)) = req.parse_size() {
        args.width = w;
        args.height = h;
    }
    if let Some(steps) = req.steps {
        args.steps = steps;
    }
    if let Some(cfg) = req.cfg_scale {
        args.cfg_scale = cfg;
    }
    if let Some(g) = req.guidance {
        args.guidance = Some(g);
    }
    if let Some(seed) = req.seed {
        args.seed = seed;
    }
    if let Some(sampler) = &req.sampler {
        args.sampling_method = sampler.clone();
    }
    if let Some(sched) = &req.scheduler {
        args.scheduler = sched.clone();
    }
    if let Some(n) = req.n {
        args.batch_count = n;
    }

    // Use temp file for output
    let temp_dir = std::env::temp_dir();
    let temp_id = uuid::Uuid::new_v4();
    let temp_path = temp_dir.join(format!("sd-{temp_id}.png"));
    args.output = temp_path.clone();

    // Serialize GPU access
    let _lock = state.gpu_lock.lock().await;

    // Run sd-cli in blocking task
    let result = tokio::task::spawn_blocking(move || args.run()).await;

    match result {
        Ok(Ok(output_path)) => {
            let image_data = match std::fs::read(&output_path) {
                Ok(data) => data,
                Err(e) => {
                    let _ = std::fs::remove_file(&output_path);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            serde_json::to_value(ErrorResponse::server_error(format!(
                                "failed to read output: {e}"
                            )))
                            .expect("error serializes"),
                        ),
                    );
                }
            };

            let _ = std::fs::remove_file(&output_path);
            let b64 =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_data);

            let created = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            (
                StatusCode::OK,
                Json(
                    serde_json::to_value(ImageGenerationResponse {
                        created,
                        data: vec![ImageData { b64_json: b64 }],
                    })
                    .expect("response serializes"),
                ),
            )
        }
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::to_value(ErrorResponse::server_error(e.to_string()))
                    .expect("error serializes"),
            ),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::to_value(ErrorResponse::server_error(format!("task join error: {e}")))
                    .expect("error serializes"),
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt as _;

    fn test_config() -> AppConfig {
        AppConfig::parse(
            r#"
sd_cli_path = "/usr/bin/true"

[models.sd15]
model = "/models/sd15.safetensors"
width = 512
height = 512
steps = 20
"#,
        )
        .expect("test config")
    }

    fn get_request(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("request builds")
    }

    fn post_json(uri: &str, body: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("request builds")
    }

    async fn body_json(resp: axum::http::Response<axum::body::Body>) -> serde_json::Value {
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("body reads");
        serde_json::from_slice(&body).expect("json parses")
    }

    #[tokio::test]
    async fn list_models_returns_configured() {
        let app = build_router(test_config());
        let resp = app
            .oneshot(get_request("/v1/models"))
            .await
            .expect("response");

        assert_eq!(resp.status(), StatusCode::OK);
        let json = body_json(resp).await;
        assert_eq!(json["object"], "list");
        assert_eq!(json["data"][0]["id"], "sd15");
        assert_eq!(json["data"][0]["object"], "model");
    }

    #[tokio::test]
    async fn generate_unknown_model_returns_404() {
        let app = build_router(test_config());
        let resp = app
            .oneshot(post_json(
                "/v1/images/generations",
                r#"{"prompt":"a cat","model":"nonexistent"}"#,
            ))
            .await
            .expect("response");

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let json = body_json(resp).await;
        assert!(
            json["error"]["message"]
                .as_str()
                .expect("has message")
                .contains("nonexistent")
        );
    }

    #[tokio::test]
    async fn generate_missing_prompt_returns_422() {
        let app = build_router(test_config());
        let resp = app
            .oneshot(post_json("/v1/images/generations", r#"{"model":"sd15"}"#))
            .await
            .expect("response");

        // axum returns 422 for missing required fields
        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
