use serde::Deserialize;
use serde::Serialize;
use warp::filters::BoxedFilter;
use warp::Filter;
use warp::Reply;

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

pub fn json<T>(source: T) -> impl Reply
where
    T: Serialize,
{
    warp::reply::json(&source)
}

///// Converts a vector of T to a vector of U then converts the U vector to a JSON reply.

//pub fn many_json<T, U>(source: Vec<T>) -> impl Reply
//where
//    U: From<T>,
//    U: Serialize,
//{
//    let target: Vec<U> = convert_vector(source);
//    warp::reply::json(&target)
//}
