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

            for (index, item) in self.questions.clone().iter().enumerate() {
                println!("\n{} - {}", index + 1, item.question.title);

                for (i, proposition) in item.question.propositions.iter().enumerate() {
                    println!("\t{} - {}", i + 1, proposition);
                }

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .unwrap_or(0);

                self.insert_response(input);
            }

            self.show_result();
        }

        fn show_result(&mut self) {
            self.calculate_result();

            let final_score =  self.score * (20 / self.total() as u8);
            match final_score {
                18..=20 => self.print_result(final_score, "Félicitations"),
                16..18 => self.print_result(final_score, "Très bien"),
                14..16 => self.print_result(final_score, "Bien"),
                12..14 => self.print_result(final_score, "Assez bien"),
                10..12 => self.print_result(final_score, "Passable"),
                8..10 => self.print_result(final_score, "Assez faible"),
                6..8 => self.print_result(final_score, "Très faible"),
                _ => self.print_result(final_score, "A revoir")
            }
        }

        fn print_result(&self, score: u8, msg: &str) {
            println!(
                "\nVotre score est de : {}/20. {}.\n",
                score,
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
