use juniper::FieldResult;

use crate::database::models as dbmodels;
use crate::graphql::*;

#[derive(GraphQLObject)]
pub struct Group {
    pub id: i32,
    pub name: String,
}


#[juniper::object(Context = Context)]
impl QueryRoot {
    fn group(&self, context: &Context, id: i32) -> FieldResult<Group> {
        dbmodels::Group::get_by_id(id, &context.connection.0)
            .map(|db_group: dbmodels::Group| Group {
                id: db_group.id,
                name: db_group.name,
            })
            .map_err(Into::into)
    }
}
