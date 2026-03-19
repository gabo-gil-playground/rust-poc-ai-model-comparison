/// logging configuration file environment variable name
pub const LOGGING_CONFIG_FILE_ENV_VAR: &str = "LOG4RS_CONFIG_FILE";
/// logging configuration file default value
pub const LOGGING_CONFIG_FILE_DEFAULT: &str = "logging_config.yaml";

/// API server host environment variable name
pub const API_SERVER_HOST_ENV_VAR: &str = "API_SERVER_HOST";
/// API server port environment variable name
pub const API_SERVER_PORT_ENV_VAR: &str = "API_SERVER_PORT";
/// API server host default value
pub const API_SERVER_HOST_DEFAULT: &str = "0.0.0.0";
/// API server port default value
pub const API_SERVER_PORT_DEFAULT: &str = "8097";
/// server running status message
pub const SERVER_RUNNING_STATUS: &str = "server is running";

/// API Health-check main path
pub const API_HEALTH_CHECK_PATH: &str = "/health";

/// API Download main path
pub const API_FOO_MAIN_PATH: &str = "/api/v1/foo";
pub const API_FOO_GET_ALL: &str = "/all";
pub const API_FOO_POST_ALL: &str = "/all";

/// HTTP client constant values
pub const HTTP_CLIENT_CONNECTION_TIMEOUT: u64 = 30;

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::constant::constants::*;

    /// Scenario:
    /// Executes constant values validation
    /// Expectation:
    /// Constant values should be validated
    #[test]
    fn when_constant_values_are_valid() {
        assert_eq!("LOG4RS_CONFIG_FILE", LOGGING_CONFIG_FILE_ENV_VAR);
        assert_eq!("logging_config.yaml", LOGGING_CONFIG_FILE_DEFAULT);

        assert_eq!("API_SERVER_HOST", API_SERVER_HOST_ENV_VAR);
        assert_eq!("API_SERVER_PORT", API_SERVER_PORT_ENV_VAR);
        assert_eq!("0.0.0.0", API_SERVER_HOST_DEFAULT);
        assert_eq!("8097", API_SERVER_PORT_DEFAULT);
        assert_eq!("server is running", SERVER_RUNNING_STATUS);

        assert_eq!("/health", API_HEALTH_CHECK_PATH);
        assert_eq!("/api/v1/foo", API_FOO_MAIN_PATH);
        assert_eq!("/all", API_FOO_GET_ALL);
        assert_eq!("/all", API_FOO_POST_ALL);

        assert_eq!(30, HTTP_CLIENT_CONNECTION_TIMEOUT);
    }
}
