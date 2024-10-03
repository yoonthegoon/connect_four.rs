use connect_four::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("63437533236115574");
    println!("{}", game);
    let score = game.eval();
    println!("score: {}", score);
    assert_eq!(score, -5);
    Ok(())
}
