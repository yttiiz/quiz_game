pub mod game_object {
    use std::io;

    #[derive(Debug)]
    pub struct Quiz {
        title: String,
        questions: [String; 4],
        score: u8,
        responses: Vec<u8>,
        result: [u8; 4],
    }

    impl Quiz {
        pub fn new(title: &str) -> Self {
            Self {
                title: String::from(title),
                questions: [
                    String::from("A - Quelle est la capitale de la Guadeloupe ?
                    1 - Pointe-à-Pitre
                    2 - Basse-Terre
                    3 - Sainte-rose"),
                    String::from("B - Combien il y a t-il d'habitants ?
                    1 - 397 861
                    2 - 412 257
                    3 - 383 559"),
                    String::from("C - Le nombre d'habitant est de 236 hab./km2 ?
                    1 - Vrai
                    2 - Faux"),
                    String::from("D - De combien d'iles est composé l'archipel ?
                    1 - 7
                    2 - 2
                    3 - 9"),
                ],
                score: 0,
                responses: vec![],
                result: [2, 1, 1, 1],
            }
        }
        
        ///Calculates the length of the results.
        fn total(&self) -> usize {
            self.result.len()
        }

        fn insert_response(&mut self, entry: String) {
            let parsing_entry = entry.trim().parse::<u8>();
            match parsing_entry {
                Ok(value) => self.responses.push(value),
                Err(_err) => ()
            }
        }
        
        fn calculate_result(&mut self) {
            for (i, num) in self.responses
            .iter()
            .enumerate() {
                if num == &self.result[i] {
                    self.score += 1;
                }
            }
        }
        
        pub fn start(&mut self) {
            println!("{}", self.title);

            for question in self.questions
            .clone()
            .iter() {
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
                true => if usize::from(self.score) == self.total() {
                    self.print_result("Félicitations !!!");
                    
                } else {
                    self.print_result("Très bien !!!");
                    self.show_good_responses();
                },
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
            println!("\nVotre score est de : {}/{}. {}\n", self.score, self.total(), msg);
        }
    }
}