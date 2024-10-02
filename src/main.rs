mod connect_four;
mod error;
mod hash;
mod prelude;
mod result;

use crate::prelude::*;

fn main() -> Result<()> {
    println!("Hello, world!");
    let game = Game::new("4453".to_string());
    println!("{:?}", game);
    Ok(())
}
