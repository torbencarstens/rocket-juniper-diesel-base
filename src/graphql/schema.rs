use std::convert::*;

use juniper::FieldResult;

use crate::database::models as dbmodels;
use crate::database::models::InsertableGroup;
use crate::graphql::*;

#[derive(GraphQLObject)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

impl Into<Group> for dbmodels::Group {
    fn into(self) -> Group {
        Group {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct InputGroup {
    pub name: String,
}

impl From<InputGroup> for dbmodels::InsertableGroup {
    fn from(input: InputGroup) -> Self {
        dbmodels::InsertableGroup {
            name: input.name
        }
    }
}

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn group(&self, context: &Context, id: i32) -> FieldResult<Group> {
        dbmodels::Group::get_by_id(id, &context.connection.0)
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn group(&self, context: &Context, input: InputGroup) -> FieldResult<Group> {
        dbmodels::InsertableGroup::from(input)
            .insert(&context.connection.0)
            .map(Into::into)
            .map_err(Into::into)
    }
}
