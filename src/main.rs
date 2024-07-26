use futures::TryStreamExt;
use quiz::{
    db_connexion::Mongo,
    game_object::Quiz
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mongo = Mongo::new().await?;
    let mut cursor = mongo.cursor().await?;
    let mut quiz = Quiz::new("Connaissez-vous la Guadeloupe ?");

    // Add questions
    while let Some(question) = cursor.try_next().await? {
        quiz.add_question(question);
    }

    quiz.start();

    Ok(())
}