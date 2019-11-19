use diesel::prelude::*;

use crate::database::schema::groups;

#[derive(Debug, Insertable)]
#[table_name = "groups"]
pub struct InsertableGroup {
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

type DieselResult<T> = Result<T, diesel::result::Error>;

impl Group {
    pub fn get_by_id(group_id: i32, connection: &diesel::PgConnection) -> DieselResult<Group> {
        use groups::dsl;

        groups::table
            .filter(dsl::id.eq(group_id))
            .first(connection)
    }
}

impl InsertableGroup {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<Group> {
        diesel::insert_into(groups::table)
            .values(self)
            .load(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
