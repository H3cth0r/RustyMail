use futures_util::TryStreamExt;
use mongodb:: {Client, Database};
use mongodb::bson::doc;
use crate::models::{User, Email};

pub struct db {
    db: Database
}

impl db {
    pub async fn new(uri: &str, db_name: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);
        Ok(Self { db })
    }

    pub async fn create_user(&self, user: &User) -> mongodb::error::Result<()> {
        let collection = self.db.collection::<User>("users");
        collection.insert_one(user).await?;
        Ok(())
    }

    pub async fn store_mail(&self, username: &str, mail: &Email) -> mongodb::error::Result<()> {
        let collection = self.db.collection::<Email>(&format!("{}_mails", username));
        collection.insert_one(mail).await?;
        Ok(())
    }

    pub async fn get_user_mails(&self, username: &str) -> mongodb::error::Result<Vec<Email>> {
        let collection = self.db.collection::<Email>(&format!("{}_mails", username));
        let mut cursor = collection.find(doc! {}).await?;
        let mut mails = Vec::new();
        while let Some(mail) = cursor.try_next().await? {
            mails.push(mail);
        }
        Ok(mails)
    }
}