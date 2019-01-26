use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use serde::{Serialize, Deserialize};

use crate::util;
use warp::path;
use warp::Filter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    oauth_token: String
}

pub fn auth_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    let login = path!("login")
        .and(warp::post2())
        .and(util::json_body_filter(1))
        .map(|login: Login| {
            println!("Got token! {}", login.oauth_token);
            // TODO should return JWT instead of "yeet"
            "yeet"
        });

    path!("auth")
        .and(
           login
        )
        .boxed()
}
