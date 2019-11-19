#![feature(proc_macro_hygiene, decl_macro)]
extern crate base;
#[macro_use]
extern crate rocket;

use base::routes;

fn main() {
    rocket::ignite()
        .mount("/",
               routes![
                    routes::index,
               ])
        .launch();
}
