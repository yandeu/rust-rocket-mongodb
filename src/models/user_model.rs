use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use struct_helpers::{to_lower_case, to_lower_case_optional, Helpers};

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Helpers)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    #[helper(to_lower_case)]
    pub name: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub bla: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Helpers)]
pub struct UserName {
    #[helper(to_lower_case)]
    pub name: String,
}

impl User {
    pub fn remove_id(&mut self) {
        self.id = None;
    }
}

impl From<UserName> for User {
    fn from(u: UserName) -> Self {
        User {
            name: u.name.into(),
            ..Default::default()
        }
    }
}
