pub mod game_object {
    use crate::db_connexion::Question;
    use std::io;

    #[derive(Debug)]
    pub struct Quiz {
        title: String,
        questions: [String; 4],
        score: u8,
        responses: Box<Vec<u8>>,
        result: [u8; 4],
        questions_db: Box<Vec<Question>>,
    }

    impl Quiz {
        pub fn new(title: &str) -> Self {
            Self {
                title: String::from(title),
                questions: [
                    String::from(
                        "A - Quelle est la capitale de la Guadeloupe ?
                    1 - Pointe-à-Pitre
                    2 - Basse-Terre
                    3 - Sainte-rose",
                    ),
                    String::from(
                        "B - Combien il y a t-il d'habitants ?
                    1 - 397 861
                    2 - 412 257
                    3 - 383 559",
                    ),
                    String::from(
                        "C - Le nombre d'habitant est de 236 hab./km2 ?
                    1 - Vrai
                    2 - Faux",
                    ),
                    String::from(
                        "D - De combien d'iles est composé l'archipel ?
                    1 - 7
                    2 - 2
                    3 - 9",
                    ),
                ],
                score: 0,
                responses: Box::new(vec![]),
                result: [2, 1, 1, 1],
                questions_db: Box::new(vec![]),
            }
        }

        pub fn add_question(&mut self, question: Question) {
            self.questions_db.push(question);
        }

        ///Calculates results length.
        fn total(&self) -> usize {
            self.result.len()
        }

        fn insert_response(&mut self, entry: String) {
            match entry.trim().parse::<u8>() {
                Ok(value) => self.responses.push(value),
                Err(_err) => (),
            }
        }

        fn calculate_result(&mut self) {
            for (i, num) in self.responses.iter().enumerate() {
                if num == &self.result[i] {
                    self.score += 1;
                }
            }
        }

        pub fn start(&mut self) {
            println!("{}", self.title);

            for question in self.questions.clone().iter() {
                println!("{}", question);

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
                        self.show_good_responses();
                    }
                }
                false => {
                    self.print_result("Vous pouvez mieux faire !!!");
                    self.show_good_responses();
                }
            }
        }

        fn show_good_responses(&self) {
            println!("Les bonnes réponses étaient :");
            println!("Question A -> Basse-Terre");
            println!("Question B -> 397 861");
            println!("Question C -> Vrai");
            println!("Question D -> 7");
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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Question {
        _id: ObjectId,
        correction: String,
        pub question: QuestionData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct QuestionData {
        pub title: String,
        propositions: Vec<String>,
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

        pub async fn cursor(&self) -> Result<Cursor<Question>, Box<dyn Error>> {
            Ok(self
                .client
                .database("quiz")
                .collection("series_001")
                .find(doc! {})
                .await?)
        }
    }
}
