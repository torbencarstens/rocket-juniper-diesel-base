#![feature(proc_macro_hygiene, decl_macro)]
extern crate base;
#[macro_use]
extern crate rocket;

use base::db::PrimaryDb;
use base::graphql::{MutationRoot, QueryRoot};
use base::routes::{self, Schema};

fn main() {
    rocket::ignite()
        .attach(PrimaryDb::fairing())
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
        ))
        .mount("/",
               routes![
                    routes::graphiql,
                    routes::get_graphql_handler,
                    routes::post_graphql_handler,
               ])
        .launch();
}
