use std::sync::Arc;

use crate::constant::constants::{API_FOO_GET_ALL, API_FOO_MAIN_PATH, API_FOO_POST_ALL};
use crate::dto::foo_get_request::FooGetRequest;
use crate::dto::foo_post_request::FooPostRequest;
use crate::service::foo_service::{DynFooService, FooService};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};

/// Foo controller
pub trait FooControllerTrait {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router;
}

/// Foo controller implementation struct
pub struct FooController {}

/// Foo controller implementation logic
impl FooControllerTrait for FooController {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router {
        let foo_service = Arc::new(FooService::default()) as DynFooService;
        Router::new()
            .nest(API_FOO_MAIN_PATH, create_routes())
            .with_state(foo_service)
    }
}

/// Creates Foo routes
fn create_routes() -> Router<DynFooService> {
    Router::new()
        .route(API_FOO_GET_ALL, get(map_get_foo))
        .route(API_FOO_POST_ALL, post(map_post_foo))
}

/// Maps foo get end-point
async fn map_get_foo(
    State(foo_service): State<DynFooService>,
    Query(foo_request): Query<FooGetRequest>,
) -> impl IntoResponse {
    match foo_service
        .get_result_from_third_party_api(foo_request.third_party_url)
        .await
    {
        Ok(foo_result) => (foo_result).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Maps foo post end-point
async fn map_post_foo(
    State(foo_service): State<DynFooService>,
    foo_request: Json<FooPostRequest>,
) -> impl IntoResponse {
    match foo_service
        .get_result_from_third_party_api(foo_request.third_party_url.clone())
        .await
    {
        Ok(foo_result) => (foo_result).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::constant::constants::{API_FOO_GET_ALL, API_FOO_POST_ALL};
    use crate::controller::foo_controller::create_routes;
    use crate::dto::foo_post_request::FooPostRequest;
    use crate::service::foo_service::{DynFooService, MockFooServiceTrait};
    use axum::http::header::CONTENT_TYPE;
    use axum::http::{Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use serde_json::json;
    use std::sync::Arc;
    use tower::util::ServiceExt;

    /// Scenario:
    /// Executes map_get_foo endpoint flow
    /// Expectation:
    /// HTTP Status 200 should be returned
    #[tokio::test]
    async fn when_map_get_foo_should_return_ok_status() {
        let mut foo_service_mock = MockFooServiceTrait::new();
        foo_service_mock
            .expect_get_result_from_third_party_api()
            .return_once(|_| Ok(String::from("some-response-value")));

        let http_request = Request::builder()
            .uri(format!(
                "{API_FOO_GET_ALL}?third_party_url=http://www.foo.com"
            ))
            .method(Method::GET)
            .body(axum::body::Body::empty())
            .unwrap();

        let foo_controller_router =
            create_routes().with_state(Arc::new(foo_service_mock) as DynFooService);

        let mut http_response = foo_controller_router.oneshot(http_request).await.unwrap();

        // http status assertion
        assert_eq!(StatusCode::OK, http_response.status());

        // http body as json assertion
        let body_as_bytes = http_response.body_mut().collect().await.unwrap().to_bytes();
        let body_as_string = String::from_utf8(body_as_bytes.to_vec()).unwrap_or_default();

        assert!(!body_as_string.is_empty());
    }

    /// Scenario:
    /// Executes map_get_foo endpoint flow when service returns an error
    /// Expectation:
    /// HTTP Status 500 should be returned
    #[tokio::test]
    async fn when_map_get_foo_when_service_return_error_should_return_error_status() {
        let mut foo_service_mock = MockFooServiceTrait::new();
        foo_service_mock
            .expect_get_result_from_third_party_api()
            .return_once(|_| Err(()));

        let http_request = Request::builder()
            .uri(format!(
                "{API_FOO_GET_ALL}?third_party_url=http://www.foo.com"
            ))
            .method(Method::GET)
            .body(axum::body::Body::empty())
            .unwrap();

        let foo_controller_router =
            create_routes().with_state(Arc::new(foo_service_mock) as DynFooService);

        let http_response = foo_controller_router.oneshot(http_request).await.unwrap();

        // http status assertion
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, http_response.status());
    }

    /// Scenario:
    /// Executes map_post_foo endpoint flow
    /// Expectation:
    /// HTTP Status 200 should be returned
    #[tokio::test]
    async fn when_map_post_foo_should_return_ok_status() {
        let mut foo_service_mock = MockFooServiceTrait::new();
        foo_service_mock
            .expect_get_result_from_third_party_api()
            .return_once(|_| Ok(String::from("some-response-value")));

        let http_body = FooPostRequest {
            third_party_url: String::from("http://www.google.com"),
        };

        let http_request = Request::builder()
            .uri(format!("{API_FOO_POST_ALL}"))
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(json!(http_body).to_string()))
            .unwrap();

        let foo_controller_router =
            create_routes().with_state(Arc::new(foo_service_mock) as DynFooService);

        let mut http_response = foo_controller_router.oneshot(http_request).await.unwrap();

        // http status assertion
        assert_eq!(StatusCode::OK, http_response.status());

        // http body as json assertion
        let body_as_bytes = http_response.body_mut().collect().await.unwrap().to_bytes();
        let body_as_string = String::from_utf8(body_as_bytes.to_vec()).unwrap_or_default();

        assert!(!body_as_string.is_empty());
    }

    /// Scenario:
    /// Executes map_post_foo endpoint flow when service returns an error
    /// Expectation:
    /// HTTP Status 500 should be returned
    #[tokio::test]
    async fn when_map_post_foo_when_service_return_error_should_return_error_status() {
        let mut foo_service_mock = MockFooServiceTrait::new();
        foo_service_mock
            .expect_get_result_from_third_party_api()
            .return_once(|_| Err(()));

        let http_body = FooPostRequest {
            third_party_url: String::from("http://www.google.com"),
        };

        let http_request = Request::builder()
            .uri(format!("{API_FOO_POST_ALL}"))
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(json!(http_body).to_string()))
            .unwrap();

        let foo_controller_router =
            create_routes().with_state(Arc::new(foo_service_mock) as DynFooService);

        let http_response = foo_controller_router.oneshot(http_request).await.unwrap();

        // http status assertion
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, http_response.status());
    }
}
