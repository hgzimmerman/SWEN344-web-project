use diesel::insertable::Insertable;
use diesel::query_builder::InsertStatement;
use diesel::pg::Pg;
use diesel::sql_types::HasSqlType;
use diesel::pg::PgConnection;
use diesel::query_builder::AsQuery;
use diesel::query_source::Queryable;
use diesel::query_builder::QueryId;
use diesel::query_builder::QueryFragment;
use diesel::query_dsl::RunQueryDsl;
use diesel::result::QueryResult;
use diesel::query_builder::AsChangeset;
use diesel::associations::HasTable;
use diesel::query_builder::IntoUpdateTarget;
use diesel::helper_types::Update;
use diesel::query_dsl::LoadQuery;
use diesel::query_source::QuerySource;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::dsl::Find;
use uuid::Uuid;
use diesel::query_builder::DeleteStatement;
use diesel::query_source::Table;
use diesel::expression::Expression;
use diesel::delete;

//#[inline(always)]
//pub fn create_row<Model, NewModel, Tab>(table: Tab, insert: NewModel, conn: &PgConnection) -> QueryResult<Model>
//where
//    NewModel: Insertable<Tab>,
//    InsertStatement<Tab, NewModel>: AsQuery,
//    Pg: HasSqlType<<InsertStatement<Tab, NewModel> as AsQuery>::SqlType>,
//    InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values>: AsQuery,
//    Model: Queryable<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType, Pg>,
//    Pg: HasSqlType<<InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::SqlType>,
//    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryId,
//    <InsertStatement<Tab, <NewModel as Insertable<Tab>>::Values> as AsQuery>::Query: QueryFragment<Pg>,
//{
//    insert
//        .insert_into(table)
//        .get_result::<Model>(conn)
//}

pub fn create_row<Model, NewModel, Table, Values>(
    table: Table,
    model_to_insert: NewModel,
    connection: &PgConnection,
) -> QueryResult<Model>
    where
        NewModel: Insertable<Table, Values=Values>,
        InsertStatement<Table, Values>: LoadQuery<PgConnection, Model>,
{
    model_to_insert.insert_into(table)
        .get_result::<Model>(connection)
}


#[inline(always)]
pub fn update_row<Model, Chg, Tab>(table: Tab, changeset: Chg, conn: &PgConnection) -> QueryResult<Model>
where
    Chg: AsChangeset<Target = <Tab as HasTable>::Table>,
    Tab: QuerySource + IntoUpdateTarget,
    Update<Tab, Chg>: LoadQuery<PgConnection, Model>,
{
    diesel::update(table)
        .set(changeset)
        .get_result::<Model>(conn)
}

/// Generic function for getting a whole row from a given table.
#[inline(always)]
pub fn get_row<'a, Model, Table>(table: Table, uuid: Uuid, conn: &PgConnection) -> QueryResult<Model>
where
    Table: FindDsl<Uuid>,
    Find<Table, Uuid>: LoadQuery<PgConnection, Model>,
{
    table.find(uuid).get_result::<Model>(conn)
}

/// Generic function for deleting a row from a given table.
#[inline(always)]
pub fn delete_row<'a, Model, Tab>(table: Tab, uuid: Uuid, conn: &PgConnection) -> QueryResult<Model>
where
    Tab: FindDsl<Uuid> + Table,
    <Tab as FindDsl<Uuid>>::Output: IntoUpdateTarget,
    Pg: HasSqlType<<<<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns as Expression>::SqlType>,
    <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryId,
    <<<Tab as FindDsl<Uuid>>::Output as HasTable>::Table as Table>::AllColumns: QueryFragment<Pg>,
    DeleteStatement<
        <<Tab as FindDsl<Uuid>>::Output as HasTable>::Table,
        <<Tab as FindDsl<Uuid>>::Output as IntoUpdateTarget>::WhereClause,
    >: LoadQuery<PgConnection, Model>,
{
    delete(table.find(uuid))
        .get_result::<Model>(conn)
}
