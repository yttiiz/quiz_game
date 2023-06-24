use quiz::game_object::Quiz;

fn main() {
    let mut quiz = Quiz::new("Connaissez-vous bien la Guadeloupe ?");
    quiz.start();
}
