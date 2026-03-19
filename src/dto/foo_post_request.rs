use serde::{Deserialize, Serialize};

/// Foo post request struct
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Clone, Default))]
pub struct FooPostRequest {
    /// third party url parameter
    pub third_party_url: String,
}

/// Unit test cases
#[cfg(test)]
mod tests {
    use crate::dto::foo_post_request::FooPostRequest;

    /// Scenario:
    /// Creates a [FooPostRequest] struct with valid values
    /// Expectation:
    /// A [FooPostRequest] with proper values should be created
    #[test]
    fn when_create_foo_post_request_with_proper_values_should_return_set_values() {
        let third_party_url = "some_value";
        let foo_post_request = FooPostRequest {
            third_party_url: String::from(third_party_url),
        };

        assert_eq!(third_party_url, foo_post_request.third_party_url);
    }
}
