use connect_four::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("1616621455211");
    println!("{}", game);
    let score = game.eval();
    println!("score: {}", score);
    // assert_eq!(score, 18);
    Ok(())
}
