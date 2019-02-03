use crate::schema;
use crate::schema::funds;
use crate::user::User;
use crate::util;
use diesel::query_dsl::QueryDsl;
use diesel::result::QueryResult;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::{Identifiable, Insertable, PgConnection, Queryable};
use uuid::Uuid;

#[derive(Clone, Debug, Identifiable, Queryable, Associations)]
#[primary_key(uuid)]
#[belongs_to(User, foreign_key = "user_uuid")]
#[table_name = "funds"]
pub struct Funds {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub quantity: f64,
}

#[derive(Insertable, Debug)]
#[table_name = "funds"]
pub struct NewFunds {
    pub user_uuid: Uuid,
    pub quantity: f64,
}

impl Funds {
    pub fn create_funds_for_user(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Funds> {
        let new_funds = NewFunds {
            user_uuid,
            quantity: 0.0,
        };
        util::create_row(schema::funds::table, new_funds, conn)
    }

    /// Sets the funds to whatever the quantity is.
    /// The calling methods should perform checks if this will make the balance negative.
    pub fn set_funds(
        user_uuid: Uuid,
        quantity: f64,
        conn: &PgConnection,
    ) -> QueryResult<Funds> {
        use crate::schema::funds::dsl as funds_dsl;
        use diesel::update;
        update(funds_dsl::funds.filter(funds_dsl::user_uuid.eq(user_uuid)))
            .set(funds_dsl::quantity.eq(quantity))
            .get_result(conn)
    }

    pub fn funds(user_uuid: Uuid, conn: &PgConnection) -> QueryResult<Funds> {
        schema::funds::dsl::funds
            .filter(schema::funds::dsl::user_uuid.eq(user_uuid))
            .first(conn)
    }
}
