use connect_four::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("211376455663355325112113664364524722".to_string());
    println!("{}", game);
    println!("{}", game.eval());
    Ok(())
}
