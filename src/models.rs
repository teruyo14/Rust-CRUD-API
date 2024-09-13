use crate::schema::rustacs;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
pub struct Rustac {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustacs)]
pub struct NewRustac {
    pub name: String,
    pub email: String,
}
