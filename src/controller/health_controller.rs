use crate::constant::constants::*;
use crate::dto::health::Health;
use axum::{Router, response::IntoResponse, routing::get};
use serde_json::json;

/// Health controller
pub trait HealthControllerTrait {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router;
}

/// Health controller implementation struct
pub struct HealthController {}

/// Health controller implementation logic
impl HealthControllerTrait for HealthController {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router {
        /// Maps health check end-point
        async fn map_health() -> impl IntoResponse {
            format!(
                "{}",
                json!(Health {
                    status: String::from(SERVER_RUNNING_STATUS)
                })
            )
        }

        Router::new().route(API_HEALTH_CHECK_PATH, get(map_health))
    }
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::constant::constants::{API_HEALTH_CHECK_PATH, SERVER_RUNNING_STATUS};
    use crate::controller::health_controller::{HealthController, HealthControllerTrait};
    use crate::dto::health::Health;
    use axum::http::{Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::util::ServiceExt;

    /// Scenario:
    /// Executes map_get_health endpoint flow
    /// Expectation:
    /// HTTP Status 200 should be returned
    #[tokio::test]
    async fn when_map_get_health_return_ok_should_return_ok_status() {
        let expected_health = Health {
            status: String::from(SERVER_RUNNING_STATUS),
        };

        let http_request = Request::builder()
            .uri(API_HEALTH_CHECK_PATH)
            .method(Method::GET)
            .body(axum::body::Body::empty())
            .unwrap();

        let mut response = HealthController::config_endpoints()
            .oneshot(http_request)
            .await
            .unwrap();

        // http status assertion
        assert_eq!(StatusCode::OK, response.status());

        // http body as json assertion
        let body_as_bytes = response.body_mut().collect().await.unwrap().to_bytes();
        let body_as_json: Value = serde_json::from_slice(&body_as_bytes).unwrap();

        assert_eq!(json!(&expected_health), body_as_json);
    }
}
