//! All database queries directly related to health metrics related to advertisement serving are contained within this module.
use crate::schema::adaptive_health;
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
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Identifiable, Queryable, Serialize, Deserialize,
)]
#[primary_key(uuid)]
#[table_name = "adaptive_health"]
pub struct HealthRecord {
    /// Unique identifier
    uuid: Uuid,
    /// The number of available servers.
    available_servers: i32,
    /// The encountered load on the servers
    load: i32,
    /// Did the server serve an advertisement under these conditions?
    did_serve: bool,
    /// The time the health record was recorded.
    time_recorded: NaiveDateTime,
}

/// A struct that facilitates the creation of `health` rows.
#[derive(Clone, Copy, Insertable, Debug, Serialize, Deserialize)]
#[table_name = "adaptive_health"]
pub struct NewHealthRecord {
    /// The number of available servers.
    pub available_servers: i32,
    /// The encountered load on the servers
    pub load: i32,
    /// Did the server serve an advertisement under these conditions?
    pub did_serve: bool,
    /// The time the health record was recorded.
    pub time_recorded: NaiveDateTime,
}

impl HealthRecord {
    /// Creates a new health record.
    pub fn create(
        new_health_record: NewHealthRecord,
        conn: &PgConnection,
    ) -> QueryResult<HealthRecord> {
        crate::util::create_row(adaptive_health::table, new_health_record, conn)
    }

    /// Gets all the health records.
    pub fn get_all(conn: &PgConnection) -> QueryResult<Vec<HealthRecord>> {
        adaptive_health::table.load(conn)
    }

    /// Gets the last 7 days of server health data.
    pub fn get_last_7_days(conn: &PgConnection) -> QueryResult<Vec<HealthRecord>> {
        let now = chrono::Utc::now().naive_utc();
        let a_week_ago = now - chrono::Duration::days(7);

        adaptive_health::table
            .filter(adaptive_health::time_recorded.gt(a_week_ago))
            .load(conn)
    }
}
