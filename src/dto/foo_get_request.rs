use serde::{Deserialize, Serialize};

/// Foo get request struct
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct FooGetRequest {
    /// third party url parameter
    pub third_party_url: String,
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::dto::foo_get_request::FooGetRequest;

    /// Scenario:
    /// Creates a [FooGetRequest] struct with valid values
    /// Expectation:
    /// A [FooGetRequest] with proper values should be created
    #[test]
    fn when_create_foo_get_request_with_proper_values_should_return_set_values() {
        let third_party_url = "some_value";
        let foo_get_request = FooGetRequest {
            third_party_url: String::from(third_party_url),
        };

        assert_eq!(third_party_url, foo_get_request.third_party_url);
    }
}
