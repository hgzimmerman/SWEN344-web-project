use crate::api::auth::TEST_CLIENT_ID;
use crate::auth::Secret;
use db::user::{NewUser, User};
use diesel::pg::PgConnection;
use testing_common::fixture::Fixture;
use uuid::Uuid;

pub struct UserFixture {
    pub user: User,
}

impl Fixture for UserFixture {
    fn generate(conn: &PgConnection) -> Self {
        let new_user = NewUser {
            client_id: TEST_CLIENT_ID.to_owned(),
        };

        let user = User::create_user(new_user, conn).unwrap();

        UserFixture { user }
    }
}
