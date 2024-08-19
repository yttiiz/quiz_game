pub mod env {
    use dotenv::dotenv;
    use std::env;

    pub fn set_variable(url: &str) -> String {
        env::var(url)
            .expect(&("You must set the \"".to_owned() + url + &"\" environment variables"))
    }

    pub fn mongo_url() -> String {
        dotenv().ok();

        let mut mongo_url = String::from("mongodb+srv://");

        // Set variables.
        mongo_url += &(set_variable("DB_USERNAME") + ":");
        mongo_url += &(set_variable("DB_PASSWORD") + "@");
        mongo_url += &(set_variable("DB_HOST"));
        mongo_url += "/?retryWrites=true&w=majority&appName=AtlasCluster";

        mongo_url
    }
}

pub mod connexion {
    use crate::utils::env::mongo_url;
    use mongodb::{
        bson::{doc, oid::ObjectId},
        Client, Cursor,
    };
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Questions {
        pub _id: ObjectId,
        pub correction: String,
        pub question: Box<QuestionData>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct QuestionData {
        pub title: String,
        pub propositions: Box<Vec<String>>,
    }

    pub struct Mongo {
        client: Client,
    }

    impl Mongo {
        pub async fn new() -> Result<Self, Box<dyn Error>> {
            Ok(Self {
                client: Client::with_uri_str(mongo_url()).await?,
            })
        }

        pub async fn cursor(&self) -> Result<Cursor<Questions>, Box<dyn Error>> {
            Ok(self
                .client
                .database("quiz")
                .collection("series_001")
                .find(doc! {})
                .await?)
        }
    }
}