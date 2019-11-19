#![feature(proc_macro_hygiene, decl_macro)]
extern crate base;
extern crate diesel;
#[macro_use]
extern crate rocket;

use diesel::prelude::*;

use base::{database::{models, schema}, routes};

fn main() {
//    rocket::ignite()
//        .mount("/",
//               routes![
//                    routes::index,
//               ])
//        .launch();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL has to be present");
    let connection: diesel::PgConnection = diesel::PgConnection::establish(
        database_url.as_str())
        .expect("Couldn't establish a connection with the database.");

    let sql_load = diesel::insert_into(schema::groups::table)
        .values(&models::InsertableGroup {
            name: "".to_string(),
        })
        .load::<models::Group>(&connection)
        .expect("Failed to insert a group");

    let group = models::Group::get_by_id(0, &connection).expect("Couldn't load group with id: 0");
    println!("{:#?}", group);
    let group = models::InsertableGroup { name: String::from("hello") }.insert(&connection).expect("Failed to insert group");
    println!("{:#?}", group);
}
