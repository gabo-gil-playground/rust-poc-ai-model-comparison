use crate::constant::constants::HTTP_CLIENT_CONNECTION_TIMEOUT;
use async_trait::async_trait;
use axum::http::{Method, Request, Uri};
use http_body_util::BodyExt;
use hyper_util::client::legacy::Client;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;

/// Foo service
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FooServiceTrait {
    /// Gets [String] value from third party api based on [String] url value
    /// Returns [()] generic error if download flow fails
    async fn get_result_from_third_party_api(&self, url: String) -> Result<String, ()>;
}

/// Foo service implementation struct
pub struct FooService {}

/// default initialization
impl Default for FooService {
    fn default() -> Self {
        FooService {}
    }
}

/// Download service implementation logic
#[async_trait]
impl FooServiceTrait for FooService {
    /// Gets [String] value from third party api based on [String] url value
    /// Returns [()] generic error if download flow fails
    async fn get_result_from_third_party_api(&self, url: String) -> Result<String, ()> {
        info!("get_result_from_third_party_api - start");

        let sanitized_url = match Uri::from_maybe_shared(url.clone()) {
            Err(_) => {
                error!("get_result_from_third_party_api - uri is not valid: {url}");
                return Err(());
            }
            Ok(valid_uri_value) => valid_uri_value,
        };

        let http_client = Client::builder(hyper_util::rt::TokioExecutor::new())
            .pool_idle_timeout(Duration::from_secs(HTTP_CLIENT_CONNECTION_TIMEOUT))
            .http2_only(false)
            .build_http();

        let http_request = Request::builder()
            .uri(sanitized_url)
            .method(Method::GET)
            .body(axum::body::Body::empty())
            .unwrap();

        match http_client.request(http_request).await {
            Ok(mut http_response) => {
                let http_response_body =
                    http_response.body_mut().collect().await.unwrap().to_bytes();
                let http_response_body_as_string =
                    String::from_utf8(http_response_body.to_vec()).unwrap_or_default();

                if !http_response.status().is_success() {
                    let http_response_status_as_string = http_response.status().to_string();
                    error!("get_result_from_third_party_api - http request is not success");
                    error!(
                        "get_result_from_third_party_api - status: {http_response_status_as_string}"
                    );
                    error!(
                        "get_result_from_third_party_api - response: {http_response_body_as_string}"
                    );

                    return Err(());
                }

                info!("get_result_from_third_party_api - done");
                Ok(http_response_body_as_string)
            }
            Err(http_error) => {
                error!("get_result_from_third_party_api - http request failed");
                error!("get_result_from_third_party_api - http request error: {http_error:?}");
                Err(())
            }
        }
    }
}

/// Foo service trait for API router state (based on Rust samples for Axum DI)
pub type DynFooService = Arc<dyn FooServiceTrait + Send + Sync>;

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::service::foo_service::{FooService, FooServiceTrait};
    use rstest::rstest;
    use wiremock::{Mock, ResponseTemplate, matchers};

    const MOCK_URL_PATH: &str = "/some-path";
    const MOCK_API_RESPONSE: &str = "some-api-response-value";

    /// Scenario:
    /// Executes get_result_from_third_party_api when url value is not valid
    /// Expectation:
    /// An [Err(())] should be returned
    #[rstest]
    #[case(String::default())]
    #[case(String::from("no valid url"))]
    #[case(String::from("no-valid-url.com"))]
    #[case(String::from("www.no-valid-url.com"))]
    #[case(String::from("ftp://no-valid-url.com"))]
    #[tokio::test]
    async fn when_get_result_from_third_party_api_and_url_is_not_valid_should_return_error(
        #[case] test_case_url: String,
    ) {
        let foo_service = FooService {};
        let result = foo_service
            .get_result_from_third_party_api(test_case_url)
            .await;

        assert_eq!((), result.unwrap_err())
    }

    /// Scenario:
    /// Executes get_result_from_third_party_api when 3rd party API response is not success
    /// Expectation:
    /// An [Err(())] should be returned
    #[rstest]
    #[case(302)]
    #[case(400)]
    #[case(401)]
    #[case(404)]
    #[case(500)]
    #[tokio::test]
    async fn when_get_result_from_third_party_api_and_response_is_not_success_should_return_error(
        #[case] test_case_status: u16,
    ) {
        let mock_server = wiremock::MockServer::start().await; // creates with random port
        mock_server
            .register(create_mock_endpoint(MOCK_URL_PATH, test_case_status, None))
            .await;

        let mock_server_host = mock_server.uri();
        let mock_api_url = format!("{mock_server_host}/{MOCK_URL_PATH}");

        let foo_service = FooService {};
        let result = foo_service
            .get_result_from_third_party_api(mock_api_url)
            .await;

        assert_eq!((), result.unwrap_err());

        mock_server.reset().await; // reset mock server and clean the context
    }

    /// Scenario:
    /// Executes get_result_from_third_party_api when 3rd party API response is success
    /// Expectation:
    /// A [String] should be returned
    #[rstest]
    #[case(200)]
    #[case(201)]
    #[case(202)]
    #[case(203)]
    #[tokio::test]
    async fn when_get_result_from_third_party_api_should_return_string(
        #[case] test_case_status: u16,
    ) {
        let mock_server = wiremock::MockServer::start().await; // creates with random port
        mock_server
            .register(create_mock_endpoint(
                MOCK_URL_PATH,
                test_case_status,
                Some(MOCK_API_RESPONSE),
            ))
            .await;

        let mock_server_host = mock_server.uri();
        let mock_api_url = format!("{mock_server_host}/{MOCK_URL_PATH}");

        let foo_service = FooService {};
        let result = foo_service
            .get_result_from_third_party_api(mock_api_url)
            .await;

        assert_eq!(MOCK_API_RESPONSE, result.unwrap_or_default());

        mock_server.reset().await; // reset mock server and clean the context
    }

    /// Scenario:
    /// Executes default function
    /// Expectation:
    /// An instance should be returned
    #[tokio::test]
    async fn when_default_should_return_an_instance() {
        use mockall::Any;
        assert!(!FooService::default().type_name().is_empty());
    }

    /// Creates [Mock] HTTP wire-mock endpoint based on [&str] path, [u16] expected status
    /// and [Option<&str>] expected response values
    pub fn create_mock_endpoint(path: &str, status: u16, response: Option<&str>) -> Mock {
        let mock_response =
            ResponseTemplate::new(status).set_body_bytes(response.unwrap_or_default());
        Mock::given(matchers::path_regex(path)).respond_with(mock_response)
    }
}
