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
                    \t1 - Pointe-à-Pitre
                    \t2 - Basse-Terre
                    \t3 - Sainte-rose"),
                    String::from("B - Combien il y a t-il d'habitants ?
                    \t1 - 397 861
                    \t2 - 412 257
                    \t3 - 383 559"),
                    String::from("C - Le nombre d'habitant est de 236 hab./km2 ?
                    \t1 - Vrai
                    \t2 - Faux"),
                    String::from("D - De combien d'iles est composé l'archipel ?
                    \t1 - 7
                    \t2 - 2
                    \t3 - 9"),
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
            for (i, num) in self.responses.iter().enumerate() {
                if num == &self.result[i] {
                    self.score += 1;
                }
            }
        }
        
        pub fn start(&mut self) {
            println!("{}", self.title);

            let questions = self.questions.clone();

            for question in questions {
                println!("{}", question);
        
                let mut input = String::new();
        
                io::stdin()
                .read_line(&mut input)
                .expect("impossible de lire ce que vous avez renseigné.");
        
                self.insert_response(input);
            }

            self.show_result();
        }

        pub fn show_result(&mut self) {
            let average = self.total() / 2;
            self.calculate_result();
            
            if usize::from(self.score) > average {
                println!("\nVotre score est de : {}/{}. Félicitations !!!!\n", self.score, self.total());
                
            } else {
                println!("\nVotre score est de : {}/{}. Vous pouvez mieux faire !!!!\n", self.score, self.total());
            }
        }
    }
}