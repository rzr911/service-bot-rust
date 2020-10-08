#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod schema;
mod websites;
mod connection;

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        // .mount("/api/v1/websites", routes![websites::handler::all, 
        // websites::handler::get,
        // websites::handler::post,
        // websites::handler::put,
        // websites::handler::delete
        // ], )
        .launch();
}
