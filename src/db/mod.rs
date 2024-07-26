use std::sync::Arc;

use mongodb:: {Client, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use async_trait::async_trait;
use futures_util::stream::TryStreamExt;
use crate::models::{User, Email};

#[async_trait]
pub trait DatabaseOperations {
    async fn connect(uri: &str) -> mongodb::error::Result<Self> where Self: Sized;
    async fn create_user(&self, user: &User) -> mongodb::error::Result<()>;
    async fn get_user(&self, email: &str) -> mongodb::error::Result<Option<User>>;
    async fn store_email(&self, email: &Email, user_email: &str, folder: &str) -> mongodb::error::Result<()>;
    async fn get_emails(&self, user_email: &str, folder: &str) -> mongodb::error::Result<Vec<Email>>;
}


#[derive(Clone)]
pub struct MongoDB {
    db: Arc<Database>
}

#[async_trait]
impl DatabaseOperations for MongoDB {
    async fn connect(uri: &str) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("zentinel_mail");
        Ok(MongoDB {
            db: Arc::new(db)
        })
    }

    async fn create_user(&self, user: &User) -> mongodb::error::Result<()> {
        let users = self.db.collection::<User>("users");
        users.insert_one(user).await?;
        
        println!("User created: {}", user.email);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> mongodb::error::Result<Option<User>> {
        let users = self.db.collection::<User>("users");
        users.find_one(doc! { "email": email }).await
    }

    async fn store_email(&self, email: &Email, user_email: &str, folder: &str) -> mongodb::error::Result<()> {
        let emails = self.db.collection::<Email>(&format!("{}_emails", user_email));
        let mut email_to_store = email.clone();
        email_to_store.folder = folder.to_string(); 
        emails.insert_one(email_to_store).await?;
        println!("Email stored for user: {}, folder: {}", user_email, folder);
        Ok(())
    }

    async fn get_emails(&self, user_email: &str, folder: &str) -> mongodb::error::Result<Vec<Email>> {
        let emails = self.db.collection::<Email>(&format!("{}_emails", user_email));
        let mut cursor = emails.find(doc! { "folder": folder }).await?;
        let mut results = Vec::new();
        while let Some(email) = cursor.try_next().await? {
            results.push(email);
        }
        Ok(results)
    }
}