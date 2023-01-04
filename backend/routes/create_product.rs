// NOTE: When a request comes in this route, 2 things will be created in the database
// 1. Product
// 2. Specification

use crate::models::products::{NewProduct, Product};
use crate::models::specifications::{NewSpecification, Specification};
use crate::models::users::User;
use crate::DbPool;
use crate::Permission;
use crate::ResponseType;
use crate::Role;
use crate::SESSION_NAME;
use actix_session::Session;
use actix_web::{
    post,
    web::{self, block, Data},
    HttpResponse, Responder,
};
use diesel::prelude::*;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Body {
    product: NewProduct,
    specification: NewSpecification,
}

#[post("/create-product")]
pub async fn create_product_route(
    pool: Data<DbPool>,
    body: web::Json<Body>,
    session: Session,
) -> impl Responder {
    // Check if the user is logged in
    if let Ok(Some(user_id)) = session.get::<i32>(SESSION_NAME) {
        // Get the user
        let user_result = {
            let pool = pool.clone();

            block(move || {
                use crate::schema::users::dsl::*;
                users
                    .filter(id.eq(user_id))
                    .first::<User>(&mut pool.get().unwrap())
            })
            .await
            .unwrap()
        };

        match user_result {
            Ok(user) => {
                println!("The user: {:?}", user);

                let user_role = Role::from(user.role);

                // Check if the user role is User ornot
                if user_role == Role::User {
                    return HttpResponse::Ok().json(ResponseType::NoPermission);
                }
                // else {
                // Check if the user role is Staff or not
                if let Role::Staff(permissions) = user_role {
                    // Check if the user has the permission to create a product or not
                    if !permissions.contains(&Permission::Create) {
                        return HttpResponse::Ok().json(ResponseType::NoPermission);
                    }
                }

                // We have to convert the `name_variable` into a string and put it in the `name` field
                // The `name_variable` can contain variables. syntax: ${variable_name}
                // The variable can any key from the specification
                // Then we have to replace the variable with the value from the specification
                // After that we will insert both the product and the specification into the database

                let new_product = body.product.clone();
                let new_specification = body.specification.clone();

                // Check if the name_variable is valid or not
                let name_variable: String = new_product.name_variable.clone();
                let re = Regex::new(r"\$\{[a-zA-Z0-9_]+\}").unwrap();
                let mut name = name_variable.clone();
                for cap in re.captures_iter(&name_variable) {
                    let variable = &cap[0];
                    let key = &variable[2..variable.len() - 1];
                    // if new_specification.contains_key(key) {
                    //     name = name.replace(variable, new_specification.get(key).unwrap());
                    // } else {
                    //     return HttpResponse::Ok().json(ResponseType::InvalidInput);
                    // }

                    // NOTE new_specification.name_value is JSONB || serde_json::Value

                    let json = new_specification.names_values.clone();
                    if json.is_object() {
                        let obj = json.as_object().unwrap();
                        if obj.contains_key(key) {
                            name = name.replace(variable, obj.get(key).unwrap().as_str().unwrap());
                        } else {
                            return HttpResponse::Ok().json(ResponseType::InvalidInput);
                        }
                    } else {
                        return HttpResponse::Ok().json(ResponseType::InvalidInput);
                    }
                }

                // Insert the product into the database
                let product_result = {
                    let var_name = name.clone();
                    let var_name_variable = name_variable.clone();
                    let pool = pool.clone();

                    block(move || {
                        use crate::schema::products::dsl::*;

                        diesel::insert_into(products)
                            .values(&NewProduct {
                                name: var_name,
                                name_variable: var_name_variable,
                                price: new_product.price,
                                image_url: new_product.image_url,
                                description: new_product.description,
                                brand: new_product.brand,
                                category_id: new_product.category_id,
                                created_by: user.id,
                                quantity: new_product.quantity,
                            })
                            .get_result::<Product>(&mut pool.get().unwrap())
                    })
                    .await
                    .unwrap()
                };

                match product_result {
                    Ok(product) => {
                        // Insert the specification into the database
                        let specification_result = block(move || {
                            use crate::schema::specifications::dsl::*;

                            diesel::insert_into(specifications)
                                .values(&NewSpecification {
                                    names_values: new_specification.names_values,
                                    product_id: product.id,
                                })
                                .get_result::<Specification>(&mut pool.get().unwrap())
                        })
                        .await
                        .unwrap();

                        match specification_result {
                            Ok(_) => HttpResponse::Ok().json(ResponseType::Success),
                            Err(e) => {
                                error!("{:?}", e);
                                HttpResponse::Ok().json(ResponseType::ServerError)
                            }
                        }
                    }
                    Err(e) => {
                        error!("{:?}", e);

                        HttpResponse::Ok().json(ResponseType::ServerError)
                    }
                }
            }
            Err(_) => HttpResponse::Ok().json(ResponseType::ServerError),
        }
    } else {
        HttpResponse::Ok().json(ResponseType::Unauthorized)
    }
}
