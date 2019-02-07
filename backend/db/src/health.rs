use crate::schema::health;
use chrono::NaiveDateTime;
use diesel::{
    pg::PgConnection,
    query_dsl::{filter_dsl::FilterDsl, RunQueryDsl},
    result::QueryResult,
    ExpressionMethods,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Server health data.
#[derive(Clone, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize)]
#[primary_key(uuid)]
#[table_name = "health"]
pub struct HealthRecord {
    uuid: Uuid,
    available_servers: i32,
    load: i32,
    did_serve: bool,
    time_recorded: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "health"]
pub struct NewHealthRecord {
    pub available_servers: i32,
    pub load: i32,
    pub did_serve: bool,
    pub time_recorded: NaiveDateTime,
}

impl HealthRecord {
    /// Creates a new health record.
    pub fn create(
        new_health_record: NewHealthRecord,
        conn: &PgConnection,
    ) -> QueryResult<HealthRecord> {
        crate::util::create_row(health::table, new_health_record, conn)
    }

    /// Gets all the health records.
    pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<HealthRecord>> {
        health::table.load(conn)
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
