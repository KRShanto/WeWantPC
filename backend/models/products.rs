use crate::schema::products;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub name_variable: String,
    pub description: String,
    pub price: i32,
    pub quantity: i32,
    pub image_url: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: i32,
    pub category_id: i32,
    pub brand: String,
}

#[derive(Insertable, Deserialize, Debug, Serialize, std::clone::Clone)]
#[diesel(table_name=products)]
pub struct NewProduct {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub name: String,
    pub name_variable: String,
    pub description: String,
    pub price: i32,
    pub quantity: i32,
    pub image_url: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub created_by: i32,
    pub category_id: i32,
    pub brand: String,
}
