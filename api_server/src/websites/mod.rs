#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::websites;
use chrono::DateTime;
use uuid::Uuid;

pub mod handler;
pub mod router;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "websites"]
pub struct Website {
    pub id: Uuid,
    pub name: String,
    // pub created_at: DateTime,
}

