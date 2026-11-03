use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
}
