pub mod user;

pub mod util {
    use crate::api::auth::LoginRequest;
    use bytes::Bytes;
    use serde::Deserialize;
    use serde_json::from_str;
    use std::ops::Deref;
    use warp::{filters::BoxedFilter, http::Response, Reply};

    /// Used in testing, this function will try to deserialize a response generated from a typical
    /// warp::testing::request() invocation.
    pub fn deserialize<T: for<'de> Deserialize<'de>>(response: Response<Bytes>) -> T {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        println!("Body string: {}", body_string);
        from_str::<T>(body_string).expect("Should be able to deserialize body")
    }

    pub fn deserialize_string(response: Response<Bytes>) -> String {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        String::from(body_string)
    }

    /// Convenience function for requesting the JWT.
    /// In the testing environment, the login function will always work.
    pub fn get_jwt(filter: BoxedFilter<(impl Reply + 'static,)>) -> String {
        let login = LoginRequest {
            oauth_token: "Test Garbage because we don't want to have the tests depend on FB"
                .to_string(),
        };

        let resp = warp::test::request()
            .method("POST")
            .path("/api/auth/login")
            .json(&login)
            .header("content-length", "300")
            .reply(&filter);

        let jwt = deserialize_string(resp);
        jwt
    }

}
