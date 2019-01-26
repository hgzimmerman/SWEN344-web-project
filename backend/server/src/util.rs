
use warp::filters::BoxedFilter;
use serde::Deserialize;
use warp::Filter;


pub fn json_body_filter<T>(kb_limit: u64) -> BoxedFilter<(T,)>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    warp::body::content_length_limit(1024 * kb_limit)
        .and(warp::body::json())
        .boxed()
}
