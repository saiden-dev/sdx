use serde::{Deserialize, Serialize};

/// OpenAI-compatible image generation request.
#[derive(Debug, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub model: Option<String>,
    /// Number of images (maps to batch_count)
    pub n: Option<u32>,
    /// Size as "WxH" string
    pub size: Option<String>,

    // Extensions beyond OpenAI spec
    pub negative_prompt: Option<String>,
    pub steps: Option<u32>,
    pub cfg_scale: Option<f32>,
    pub guidance: Option<f32>,
    pub seed: Option<i64>,
    pub sampler: Option<String>,
    pub scheduler: Option<String>,
}

/// OpenAI-compatible image generation response.
#[derive(Debug, Serialize)]
pub struct ImageGenerationResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Serialize)]
pub struct ImageData {
    pub b64_json: String,
}

/// OpenAI-compatible model listing response.
#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub object: &'static str,
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: &'static str,
    pub owned_by: &'static str,
}

/// OpenAI-compatible error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
}

impl ImageGenerationRequest {
    pub fn parse_size(&self) -> Option<(u32, u32)> {
        let s = self.size.as_ref()?;
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return None;
        }
        let w = parts[0].parse().ok()?;
        let h = parts[1].parse().ok()?;
        Some((w, h))
    }
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            error: ErrorBody {
                message: message.into(),
                error_type: "invalid_request_error".into(),
                code: None,
            },
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            error: ErrorBody {
                message: message.into(),
                error_type: "not_found_error".into(),
                code: Some("model_not_found".into()),
            },
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self {
            error: ErrorBody {
                message: message.into(),
                error_type: "server_error".into(),
                code: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_size_valid() {
        let req = ImageGenerationRequest {
            prompt: "test".into(),
            model: None,
            n: None,
            size: Some("512x768".into()),
            negative_prompt: None,
            steps: None,
            cfg_scale: None,
            guidance: None,
            seed: None,
            sampler: None,
            scheduler: None,
        };
        assert_eq!(req.parse_size(), Some((512, 768)));
    }

    #[test]
    fn parse_size_none() {
        let req = ImageGenerationRequest {
            prompt: "test".into(),
            model: None,
            n: None,
            size: None,
            negative_prompt: None,
            steps: None,
            cfg_scale: None,
            guidance: None,
            seed: None,
            sampler: None,
            scheduler: None,
        };
        assert_eq!(req.parse_size(), None);
    }

    #[test]
    fn parse_size_invalid_format() {
        let req = ImageGenerationRequest {
            prompt: "test".into(),
            model: None,
            n: None,
            size: Some("512".into()),
            negative_prompt: None,
            steps: None,
            cfg_scale: None,
            guidance: None,
            seed: None,
            sampler: None,
            scheduler: None,
        };
        assert_eq!(req.parse_size(), None);
    }

    #[test]
    fn parse_size_non_numeric() {
        let req = ImageGenerationRequest {
            prompt: "test".into(),
            model: None,
            n: None,
            size: Some("abcxdef".into()),
            negative_prompt: None,
            steps: None,
            cfg_scale: None,
            guidance: None,
            seed: None,
            sampler: None,
            scheduler: None,
        };
        assert_eq!(req.parse_size(), None);
    }

    #[test]
    fn error_response_new() {
        let resp = ErrorResponse::new("bad request");
        assert_eq!(resp.error.message, "bad request");
        assert_eq!(resp.error.error_type, "invalid_request_error");
        assert!(resp.error.code.is_none());
    }

    #[test]
    fn error_response_not_found() {
        let resp = ErrorResponse::not_found("model missing");
        assert_eq!(resp.error.error_type, "not_found_error");
        assert_eq!(resp.error.code.as_deref(), Some("model_not_found"));
    }

    #[test]
    fn error_response_server_error() {
        let resp = ErrorResponse::server_error("internal");
        assert_eq!(resp.error.error_type, "server_error");
    }

    #[test]
    fn image_generation_response_serializes() {
        let resp = ImageGenerationResponse {
            created: 1234567890,
            data: vec![ImageData {
                b64_json: "abc123".into(),
            }],
        };
        let json = serde_json::to_string(&resp).expect("serializes");
        assert!(json.contains("\"created\":1234567890"));
        assert!(json.contains("\"b64_json\":\"abc123\""));
    }

    #[test]
    fn models_response_serializes() {
        let resp = ModelsResponse {
            object: "list",
            data: vec![ModelInfo {
                id: "sd15".into(),
                object: "model",
                owned_by: "local",
            }],
        };
        let json = serde_json::to_string(&resp).expect("serializes");
        assert!(json.contains("\"id\":\"sd15\""));
        assert!(json.contains("\"object\":\"list\""));
    }

    #[test]
    fn image_generation_request_deserializes() {
        let json = r#"{
            "prompt": "a cat",
            "model": "sd15",
            "n": 2,
            "size": "768x768",
            "negative_prompt": "ugly",
            "steps": 30,
            "cfg_scale": 5.0,
            "guidance": 3.5,
            "seed": 42,
            "sampler": "euler",
            "scheduler": "simple"
        }"#;
        let req: ImageGenerationRequest = serde_json::from_str(json).expect("deserializes");
        assert_eq!(req.prompt, "a cat");
        assert_eq!(req.model.as_deref(), Some("sd15"));
        assert_eq!(req.n, Some(2));
        assert_eq!(req.steps, Some(30));
        assert_eq!(req.seed, Some(42));
    }

    #[test]
    fn image_generation_request_minimal() {
        let json = r#"{"prompt": "hello"}"#;
        let req: ImageGenerationRequest = serde_json::from_str(json).expect("deserializes");
        assert_eq!(req.prompt, "hello");
        assert!(req.model.is_none());
        assert!(req.size.is_none());
    }
}
