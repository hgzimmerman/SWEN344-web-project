//! Common utilities
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter, Reply};

/// Extracts the body of a request after stipulating that it has a reasonable size in kilobytes.
pub fn json_body_filter<T>(kb_limit: u64) -> BoxedFilter<(T,)>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    warp::body::content_length_limit(1024 * kb_limit)
        .and(warp::body::json())
        .boxed()
}

#[allow(dead_code)]
/// Util function that makes replying easier
pub fn json_convert<T, U>(source: T) -> impl Reply
where
    T: Into<U>,
    U: Serialize,
{
    let target: U = source.into();
    warp::reply::json(&target)
}

/// Converts a serializable type to a JSON reply.
pub fn json<T>(source: T) -> impl Reply
where
    T: Serialize,
{
    warp::reply::json(&source)
}

/// Converts a vector of T to a vector of U then converts the U vector to a JSON reply.
#[allow(dead_code)]
pub fn many_json_converts<T, U>(source: impl IntoIterator<Item = T>) -> impl Reply
where
    U: From<T>,
    U: Serialize,
{
    let target: Vec<U> = source.into_iter().map(U::from).collect();
    warp::reply::json(&target)
}
