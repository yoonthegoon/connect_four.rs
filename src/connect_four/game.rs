use crate::prelude::*;
use std::fmt::{Display, Formatter};

/// ```rs
/// let game = Game::new("445".to_string());
/// // game = Game { position: "445", grid: 270532608, mask: 274726912, state: NonTerminal }
///
/// let game = game.play("3").unwrap();
/// // game = Game { position: "4453", grid: 4210688, mask: 274743296, state: NonTerminal }
///
/// ```
/// ```
///         grid                    mask
///  0  0  0  0  0  0  0     0  0  0  0  0  0  0
///  0  0  0  0  0  0  0     0  0  0  0  0  0  0
///  0  0  0  0  0  0  0     0  0  0  0  0  0  0
///  0  0  0  0  0  0  0     0  0  0  0  0  0  0
///  0  0  0  0  0  0  0     0  0  0  0  0  0  0
///  0  0  0  1  0  0  0     0  0  0  1  0  0  0
///  0  0  1  0  0  0  0     0  0  1  1  1  0  0
///
///         bits                   display
///  6 13 20 27 34 41 48     1  2  3  4  5  6  7
///  5 12 19 26 33 40 47     .  .  .  .  .  .  .
///  4 11 18 25 32 39 46     .  .  .  .  .  .  .
///  3 10 17 24 31 38 45     .  .  .  .  .  .  .
///  2  9 16 23 30 37 44     .  .  .  .  .  .  .
///  1  8 15 22 29 36 43     .  .  .  O  .  .  .
///  0  7 14 21 28 35 42     .  .  O  X  X  .  .
/// ```
#[derive(Debug)]
pub struct Game {
    position: String,
    grid: u64,
    mask: u64,
    state: State,
}

impl Game {
    pub fn new(position: String) -> Self {
        if position.len() == 0 {
            return Game {
                position,
                grid: 0,
                mask: 0,
                state: NonTerminal,
            };
        }

        let (pos_str, move_) = position.split_at(position.len() - 1);
        let grid = Game::new(pos_str.to_string());
        grid.play(move_).unwrap()
    }

    fn get_state(grid: u64, mask: u64) -> State {
        for direction in [1, 6, 7, 8] {
            let intersected = grid << direction & grid;
            if intersected << 2 * direction & intersected != 0 { return Win; }
        }
        if mask & 279258638311359 == 279258638311359 { Draw } else { NonTerminal }
    }

    /// returns new game where move_ was played
    /// error if game.state is terminal, column filled, or wrong value passed in to move_
    fn play(&self, move_: &str) -> Result<Self> {
        match self.state {
            NonTerminal => {}
            Win | Draw => { return Err("self.state is terminal; game has concluded".into()) }
        }
        let mask;
        match move_.parse::<usize>() {
            Ok(0) => return Err(ValueError("play(move_: &str) expects digit 1-7; got 0".to_string())),
            Ok(a) => {
                if a > 7 { return Err(ValueError(format!("play(move_: &str) expects digit 1-7; got {}", move_))); }
                let x = 1 << 7 * (a - 1);
                let no_go = x << 5;
                if self.mask & no_go == no_go { return Err(format!("column {} already filled", move_).into()); }
                mask = self.mask + x | self.mask;
            }
            Err(e) => return Err(e.into()),
        }

        let mut position = self.position.clone();
        position.push_str(move_);
        let grid = self.grid ^ mask;
        let state = Self::get_state(grid, mask);

        let grid = Game {
            position,
            grid,
            mask,
            state,
        };
        Ok(grid)
    }

    /// returns Player to play
    fn to_play(&self) -> Player { (self.position.len() % 2).into() }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\x1b[4m 1  2  3  4  5  6  7 \x1b[0m")?;
        let p64 = self.to_play() as u64;
        for m in (0..6).rev() {
            for n in 0..7 {
                let space = 1 << m << 7 * n;
                if self.mask & (1 << m << 7 * n) == 0 {
                    write!(f, " . ")?;
                    continue;
                }
                match p64 == (self.grid & space) >> 7 * n >> m {
                    true => write!(f, "\x1b[31m X \x1b[0m")?,
                    false => write!(f, "\x1b[33m O \x1b[0m")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "\x1b[4m")?;
        match self.state {
            NonTerminal => match p64.into() {
                Player1 => writeln!(f, "to play:\x1b[31m X \x1b[0m")?,
                Player2 => writeln!(f, "to play:\x1b[33m O \x1b[0m")?,
            }
            Win => match p64.into() {
                Player1 => writeln!(f, "winner:\x1b[33m O \x1b[0m")?,
                Player2 => writeln!(f, "winner:\x1b[31m X \x1b[0m")?,
            }
            Draw => writeln!(f, "game drawn")?
        }
        writeln!(f, "\x1b[0m")?;
        Ok(())
    }
}
