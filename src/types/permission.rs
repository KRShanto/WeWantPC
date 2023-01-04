use crate::*;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    Create,
    Read,
    Update,
    Delete,
}

impl From<String> for Permission {
    fn from(permission: String) -> Self {
        match permission.as_str() {
            "create" | "Create" => Permission::Create,
            "read" | "Read" => Permission::Read,
            "update" | "Update" => Permission::Update,
            "delete" | "Delete" => Permission::Delete,
            _ => Permission::Read,
        }
    }
}

impl From<Permission> for String {
    fn from(permission: Permission) -> Self {
        match permission {
            Permission::Create => "Create".to_string(),
            Permission::Read => "Read".to_string(),
            Permission::Update => "Update".to_string(),
            Permission::Delete => "Delete".to_string(),
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Create => write!(f, "Create"),
            Permission::Read => write!(f, "Read"),
            Permission::Update => write!(f, "Update"),
            Permission::Delete => write!(f, "Delete"),
        }
    }
}
