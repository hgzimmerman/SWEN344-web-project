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
use crate::util::json_body_filter;
use diesel::result::QueryResult;

/// The user api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn user_api(state: &State) -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Auth api");

    let get_zip_code = path!("zip")
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(|user_uuid: Uuid, conn: PooledConn| -> QueryResult<Option<String>> {
            User::get_zip_code(user_uuid, &conn)
        })
        .and_then(util::json_or_reject);

    let set_zip_code = path!("zip")
        .and(warp::put2())
        .and(json_body_filter(1))
        .and(user_filter(state))
        .and(state.db())
        .map(|zip_code: String, user_uuid: Uuid, conn: PooledConn| {
            User::set_zip_code(user_uuid, zip_code, &conn)
        })
        .and_then(util::json_or_reject);

    let get_user = warp::get2()
        .and(user_filter(state))
        .and(state.db())
        .map(|user_uuid: Uuid, conn: PooledConn| -> QueryResult<User> {
            User::get_user(user_uuid, &conn)
        })
        .and_then(util::json_or_reject);

    path!("user")
        .and(
            get_user
               .or(set_zip_code)
               .or(get_zip_code)
        )
        .boxed()

}



