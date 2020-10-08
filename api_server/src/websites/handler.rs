use connection::DbConn;
use diesel::result::Error;
use std::env;
use websites;
use websites::Website;
use rocket::http::Status;
use rocket::response::status;
use rocket::outcome::Outcome;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

// #[get("/")]
// pub fn all(connection: DbConn) -> <Json<Vec<Website>> {
//     websites::repository::all(&connection)
//         .map(|websites| Json(websites))
//         // .map_err(|error| error_status(error))
// }

// fn error_status(error: Error) -> Failure {
//     Failure(match error {
//         Error::NotFound => Status::NotFound,
//         _ => Status::InternalServerError
//     })
// }

// #[get("/<id>")]
// pub fn get(id: Uuid, connection: DbConn) -> Result<Json<Website>, Failure> {
//     websites::repository::get(id, &connection)
//         .map(|website| Json(website))
//         .map_err(|error| error_status(error))
// }

// #[post("/", format = "application/json", data = "<website>")]
// pub fn post(website: Json<Website>, connection: DbConn) -> Result<status::Created<Json<Website>>, Failure> {
//     websites::repository::insert(website.into_inner(), &connection)
//         .map(|website| website_created(website))
//         .map_err(|error| error_status(error))
// }

// fn website_created(website: Website) -> status::Created<Json<Website>> {
//     let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
//     let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");
//     status::Created(
//         format!("{host}:{port}/websites/{id}", host = host, port = port, id = website.id).to_string(),
//         Some(Json(website)))
// }

// #[put("/<id>", format = "application/json", data = "<website>")]
// pub fn put(id: Uuid, website: Json<Website>, connection: DbConn) -> Result<Json<Website>, Failure> {
//     websites::repository::update(id, website.into_inner(), &connection)
//         .map(|website| Json(website))
//         .map_err(|error| error_status(error))
// }

// #[delete("/<id>")]
// pub fn delete(id: Uuid, connection: DbConn) -> Result<status::NoContent, Failure> {
//     match websites::repository::get(id, &connection) {
//         Ok(_) => websites::repository::delete(id, &connection)
//             .map(|_| status::NoContent)
//             .map_err(|error| error_status(error)),
//         Err(error) => Err(error_status(error))
//     }
// }