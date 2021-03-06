use crate::{
    server_auth::user_filter,
    state::State,
    util::{self, json_body_filter},
};
use db::user::User;
use diesel::result::QueryResult;
use log::info;
use pool::PooledConn;
use uuid::Uuid;
use warp::{path, Filter, Rejection, Reply};

/// The user api.
///
/// # Arguments
/// state - State object reference required for accessing db connections, auth keys,
/// and other stateful constructs.
pub fn user_api(state: &State) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    info!("Attaching User api");

    let get_zip_code = path!("zip")
        .and(warp::get2())
        .and(user_filter(state))
        .and(state.db())
        .map(
            |user_uuid: Uuid, conn: PooledConn| -> QueryResult<Option<String>> {
                info!("getting zip code for user: {}", user_uuid);
                User::get_zip_code(user_uuid, &conn)
            },
        )
        .and_then(util::json_or_reject);

    let set_zip_code = path!("zip")
        .and(warp::put2())
        .and(json_body_filter(1))
        .and(user_filter(state))
        .and(state.db())
        .map(|zip_code: String, user_uuid: Uuid, conn: PooledConn| {
            info!("Setting zip code to {} for user: {}", zip_code, user_uuid);
            User::set_zip_code(user_uuid, zip_code, &conn)
        })
        .and_then(util::json_or_reject);

    let get_user = warp::get2()
        .and(warp::path::end())
        .and(user_filter(state))
        .and(state.db())
        .map(|user_uuid: Uuid, conn: PooledConn| -> QueryResult<User> {
            User::get_user(user_uuid, &conn)
        })
        .and_then(util::json_or_reject);

    path!("user").and(get_user.or(set_zip_code).or(get_zip_code))
}
