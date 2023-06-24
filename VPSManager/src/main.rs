#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
mod models;
mod database;

fn main() {
    rocket::ignite()
        .manage(database::connect())
        .mount("/", routes![
            routes::create_vps,
            routes::get_vps,
            routes::update_vps,
            routes::delete_vps,
        ])
        .launch();
}
