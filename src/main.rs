use connect_four::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("2252576253462244111563365343671351441");
    println!("{}", game);
    let score = game.eval();
    println!("score: {}", score);
    assert_eq!(score, -1);
    Ok(())
}
