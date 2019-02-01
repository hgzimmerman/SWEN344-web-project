use db::user::{User, NewUser};
use crate::auth::Secret;
use testing_common::fixture::Fixture;
use diesel::pg::PgConnection;
use uuid::Uuid;
use crate::api::auth::TEST_CLIENT_ID;


pub struct UserFixture {
    pub user: User,
}


impl Fixture for UserFixture {
    fn generate(conn: &PgConnection) -> Self {
        let new_user = NewUser {
            client_id: TEST_CLIENT_ID.to_owned()
        };

        let user = User::create_user(new_user, conn).unwrap();

        UserFixture {
            user
        }
    }
}
