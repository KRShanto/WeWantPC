use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub password: String,
    pub role: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub verified: Option<bool>,
    pub img_url: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub role: String,
}
