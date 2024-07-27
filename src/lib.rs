pub mod game_object {
    use crate::db_connexion::Questions;
    use std::io;

    #[derive(Debug)]
    pub struct Quiz {
        title: String,
        score: u8,
        responses: Box<Vec<u8>>,
        questions: Box<Vec<Questions>>,
    }

    impl Quiz {
        pub fn new(title: &str) -> Self {
            Self {
                title: String::from(title),
                score: 0,
                responses: Box::new(vec![]),
                questions: Box::new(vec![]),
            }
        }

        pub fn add_question(&mut self, question: Questions) {
            self.questions.push(question);
        }

        ///Calculates results length.
        fn total(&self) -> usize {
            self.questions.len()
        }

        fn insert_response(&mut self, entry: String) {
            match entry.trim().parse::<u8>() {
                Ok(value) => self.responses.push(value),
                Err(_err) => (),
            }
        }

        fn calculate_result(&mut self) {
            for (i, num) in self.responses.iter().enumerate() {
                let questions = self.questions[i].clone();
                let response_expected = questions.correction;
                let index = questions
                    .question
                    .propositions
                    .iter()
                    .position(|r| r == &response_expected)
                    .unwrap();

                if usize::from(num - 1) == index {
                    self.score += 1;
                }
            }
        }

        pub fn start(&mut self) {
            println!("{}", self.title);

            for item_question in self.questions.clone().iter() {
                println!("{}", item_question.question.title);

                for (i, proposition) in item_question.question.propositions.iter().enumerate() {
                    println!("{} - {}", i + 1, proposition);
                }

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("impossible de lire ce que vous avez renseigné");

                self.insert_response(input);
            }

            self.show_result();
        }

        fn show_result(&mut self) {
            let average = self.total() / 2;
            self.calculate_result();

            match usize::from(self.score) > average {
                true => {
                    if usize::from(self.score) == self.total() {
                        self.print_result("Félicitations !!!");
                    } else {
                        self.print_result("Très bien !!!");
                    }
                }
                false => {
                    self.print_result("Vous pouvez mieux faire !!!");
                }
            }
        }

        fn print_result(&self, msg: &str) {
            println!(
                "\nVotre score est de : {}/{}. {}\n",
                self.score,
                self.total(),
                msg
            );
        }
    }
}

pub mod env_variables {
    use dotenv::dotenv;
    use std::env;

    fn set_variable(url: &str) -> String {
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

pub mod db_connexion {
    use crate::env_variables::mongo_url;
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
        pub question: QuestionData,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct QuestionData {
        pub title: String,
        pub propositions: Vec<String>,
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
