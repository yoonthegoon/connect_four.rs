use connect_four::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("52677675164321472411331752454");
    println!("{}", game);
    let score = game.eval();
    println!("score: {}", score);
    assert_eq!(score, 0);
    Ok(())
}
