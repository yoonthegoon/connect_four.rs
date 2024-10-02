mod connect_four;
mod error;
mod hash;
mod prelude;
mod result;

use crate::prelude::*;

fn main() -> Result<()> {
    let game = Game::new("4453".to_string());
    println!("{}", game);
    Ok(())
}
