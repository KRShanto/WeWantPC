use crate::*;
use log::error;
use std::fmt::{Display, Formatter};

/// Roles for maintaining an ecommerce site
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
    Staff(Vec<Permission>),
}

// String -> Role
impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" | "Admin" => Role::Admin,
            "user" | "User" => Role::User,
            _ => {
                // Check if the role starts with Staff or staff or not
                if role.starts_with("Staff") || role.starts_with("staff") {
                    // Get the permissions
                    let permissions = role
                        .split('(')
                        .last()
                        .unwrap()
                        .split(')')
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|permission| Permission::from(permission.to_string()))
                        .collect();

                    Role::Staff(permissions)
                } else {
                    error!("Invalid role: {}", role);
                    Role::User
                }
            }
        }
    }
}

// Role -> String
impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Admin => "Admin".to_string(),
            Role::User => "User".to_string(),
            Role::Staff(permissions) => {
                // Staff(..permissions,)
                let mut staff = "Staff(".to_string();
                for permission in permissions {
                    staff.push_str(&permission.to_string());
                    staff.push(',');
                }
                staff.pop();
                staff.push(')');

                staff
            }
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
            Role::Staff(permissions) => {
                let mut staff = "Staff(".to_string();
                for permission in permissions {
                    staff.push_str(&permission.to_string());
                    staff.push(',');
                }
                staff.pop();
                staff.push(')');

                write!(f, "{}", staff)
            }
        }
    }
}

#[test]
// write a basic test for the Role enum
// TODO: imporve it
fn test_role() {
    let role = Role::from("Admin".to_string());
    assert_eq!(role.to_string(), "Admin".to_string());

    let role = Role::from("User".to_string());
    assert_eq!(role.to_string(), "User".to_string());

    let role = Role::from("staff(Read,Write)".to_string());
    assert_eq!(role.to_string(), "Staff(Read,Write)".to_string());
}
