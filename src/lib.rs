pub mod utils;

pub mod game_object {
    use std::io;
    use crate::utils::connexion::Questions;

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
                Err(_) => self.responses.push(0),
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

                if usize::from(*num) == index + 1 {
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
