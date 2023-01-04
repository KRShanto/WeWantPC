use crate::*;

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
pub enum ResponseType {
    AlreadyExists,
    Success,
    NotFound,
    ServerError,
    NoPermission,
    InvalidInput,
    Unauthorized,
}
