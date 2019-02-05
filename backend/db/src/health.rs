use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::health;
use serde::{Serialize, Deserialize};
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use diesel::query_dsl::RunQueryDsl;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::ExpressionMethods;


/// Server health data.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "health"]
pub struct HealthRecord {
    uuid: Uuid,
    available_servers: i32,
    load: i32,
    did_serve: bool,
    time_recorded: NaiveDateTime
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "health"]
pub struct NewHealthRecord {
    available_servers: i32,
    load: i32,
    did_serve: bool,
    time_recorded: NaiveDateTime
}

impl HealthRecord {
    /// Creates a new health record.
    pub fn create(new_health_record: NewHealthRecord, conn: &PgConnection) -> QueryResult<HealthRecord> {
        crate::util::create_row(health::table, new_health_record, conn)
    }

    /// Gets all the health records.
    pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<HealthRecord>> {
        health::table
            .load(conn)
    }

    /// Gets the last 7 days of server health data.
    pub fn get_last_7_days(conn: &PgConnection) -> QueryResult<Vec<HealthRecord>> {
        let now = chrono::Utc::now().naive_utc();
        let a_week_ago = now - chrono::Duration::days(7);

        health::table
            .filter(health::time_recorded.gt(a_week_ago))
            .load(conn)

    }
}