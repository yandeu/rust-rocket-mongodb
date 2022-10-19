use std::{env, error::Error};
extern crate dotenv;
use dotenv::dotenv;
use rocket::futures::StreamExt;

use crate::models::user_model::User;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();

        let mut uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let host = env::var("MONGODB_HOST").unwrap_or("localhost".to_string());
        let port = env::var("MONGODB_PORT").unwrap_or("27017".to_string());

        uri = uri.replace("HOST", &host);
        uri = uri.replace("PORT", &port);

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }
}

impl MongoRepo {
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Box<dyn Error>> {
        let user = match self.col.insert_one(new_user, None).await {
            Ok(u) => u,
            Err(e) => {
                print!("{}", e);
                panic!("{}", e)
            }
        };

        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(
        &self,
        id: &String,
        new_user: User,
    ) -> Result<UpdateResult, Box<dyn Error>> {
        // do never update "_id"
        let mut doc = to_document(&new_user).unwrap();
        doc.remove("_id");

        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! { "$set": doc };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let users: Vec<User> = match self.col.find(None, None).await {
            Ok(cursors) => cursors.map(|doc| doc.unwrap()).collect().await,
            Err(_e) => {
                println!("ERROR: Error getting list of users");
                Vec::new()
            }
        };

        Ok(users)
    }
}
