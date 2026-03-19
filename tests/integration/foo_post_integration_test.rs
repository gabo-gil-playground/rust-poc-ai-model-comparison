mod foo_map_post_it {
    use axum::http::header::CONTENT_TYPE;
    use axum::http::{Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use rust_poc_ai_model_comparison::constant::constants::{
        API_FOO_MAIN_PATH, API_FOO_POST_ALL,
    };
    use rust_poc_ai_model_comparison::controller::foo_controller::{
        FooController, FooControllerTrait,
    };
    use rust_poc_ai_model_comparison::dto::foo_post_request::FooPostRequest;
    use serde_json::json;
    use serial_test::serial;
    use tower::ServiceExt;
    use wiremock::{Mock, ResponseTemplate, matchers};

    const MOCK_3RD_PARTY_API_PATH: &str = "/some-path";
    const MOCK_3RD_PARTY_API_RESPONSE: &str = "some-api-response-value";

    /// Scenario:
    /// Executes map_get_foo flow with valid parameters
    /// Expectation:
    /// A [StatusCode::OK] value should be returned
    #[tokio::test]
    #[serial] // run in order just to avoid race conditions in the context
    async fn when_map_get_foo_should_return_http_ok_status() {
        let mock_server = wiremock::MockServer::start().await; // creates with random port
        mock_server
            .register(create_mock_endpoint(
                MOCK_3RD_PARTY_API_PATH,
                200,
                Some(MOCK_3RD_PARTY_API_RESPONSE),
            ))
            .await;

        let mock_server_host = mock_server.uri();
        let mock_api_url = format!("{mock_server_host}/{MOCK_3RD_PARTY_API_PATH}");

        let http_body = FooPostRequest {
            third_party_url: mock_api_url,
        };

        let http_request = Request::builder()
            .uri(format!("{API_FOO_MAIN_PATH}{API_FOO_POST_ALL}"))
            .method(Method::POST)
            .header(CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(json!(http_body).to_string()))
            .unwrap();

        let mut http_response = FooController::config_endpoints()
            .oneshot(http_request)
            .await
            .unwrap();

        // http status assertion
        assert_eq!(StatusCode::OK, http_response.status());

        // http body as json assertion
        let body_as_bytes = http_response.body_mut().collect().await.unwrap().to_bytes();
        let body_as_string = String::from_utf8(body_as_bytes.to_vec()).unwrap_or_default();

        assert_eq!(MOCK_3RD_PARTY_API_RESPONSE, body_as_string);

        mock_server.reset().await; // reset mock server and clean the context
    }

    /// Creates [Mock] HTTP wire-mock endpoint based on [&str] path, [u16] expected status
    /// and [Option<&str>] expected response values
    pub fn create_mock_endpoint(path: &str, status: u16, response: Option<&str>) -> Mock {
        let mock_response =
            ResponseTemplate::new(status).set_body_bytes(response.unwrap_or_default());
        Mock::given(matchers::path_regex(path)).respond_with(mock_response)
    }
}
