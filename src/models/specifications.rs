use crate::schema::specifications;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Specification {
    pub id: i32,
    // Nullable<Jsonb>
    // name is title
    // value is description
    pub names_values: Option<serde_json::Value>,
    pub product_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = specifications)]
pub struct NewSpecification {
    pub names_values: serde_json::Value,
    pub product_id: i32,
}
