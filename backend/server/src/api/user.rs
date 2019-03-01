use warp::path;
use warp::Filter;
use crate::server_auth::user_filter;
use crate::state::State;
use warp::filters::BoxedFilter;
use warp::Reply;
use pool::PooledConn;
use uuid::Uuid;
use db::user::User;
use crate::util;
use log::info;

/// The user api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn user_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Auth api");


    path!("user")
        .and(user_filter(state))
        .and(state.db.clone())
        .map(|user_uuid: Uuid, conn: PooledConn| {
            User::get_user(user_uuid, &conn)
        })
        .and_then(util::json_or_reject)
        .boxed()

}



