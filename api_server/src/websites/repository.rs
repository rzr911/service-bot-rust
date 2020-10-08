use diesel;
use diesel::prelude::*;
use schema::websites;
use websites::Website;
use uuid::Uuid;
// use rocket_contrib::uuid::Uuid;
// use diesel::pg::types::sql_types::Uuid;


pub fn all(connection: &PgConnection) -> QueryResult<Vec<Website>> {
    websites::table.load::<Website>(&*connection)
}

// pub fn get(id: Uuid, connection: &PgConnection) -> QueryResult<Website> {
//     websites::table.find(id).get_result::<Website>(connection)
// }

// pub fn insert(website: Website, connection: &PgConnection) -> QueryResult<Website> {
//     diesel::insert_into(websites::table)
//         .values(&InsertableWebsite::from_website(website))
//         .get_result(connection)
// }

// pub fn update(id: Uuid, website: Website, connection: &PgConnection) -> QueryResult<Website> {
//     diesel::update(websites::table.find(id))
//         .set(&website)
//         .get_result(connection)
// }

// pub fn delete(id: Uuid, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(websites::table.find(id))
//         .execute(connection)
// }


#[derive(Insertable)]
#[table_name = "websites"]
struct InsertableWebsite {
    name: String,
}

impl InsertableWebsite {

    fn from_website(website: Website) -> InsertableWebsite {
        InsertableWebsite {
            name: website.name,
        }
    }
}