pub mod user;

pub mod util {
    use bytes::Bytes;
    use serde::Deserialize;
    use serde_json::from_str;
    use std::ops::Deref;
    use warp::{http::Response};

    /// Used in testing, this function will try to deserialize a response generated from a typical
    /// warp::testing::request() invocation.
    pub fn deserialize<T: for<'de> Deserialize<'de>>(response: Response<Bytes>) -> T {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        println!("Body string: {}", body_string);
        from_str::<T>(body_string).expect("Should be able to deserialize body")
    }

    #[allow(unused)]
    pub fn deserialize_string(response: Response<Bytes>) -> String {
        let body = response.into_body();
        let bytes: &[u8] = body.deref();
        let body_string = std::str::from_utf8(bytes).expect("valid utf8 string");
        String::from(body_string)
    }



}
