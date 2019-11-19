#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod database;
pub mod graphql;
pub mod routes;
mod db;
