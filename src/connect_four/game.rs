use crate::prelude::*;
use std::fmt::{Display, Formatter};

/// ```
/// use connect_four::prelude::*;
///
/// let game1 = Game::new("445".to_string()).play("3").unwrap();
/// let game2 = Game::new("4453".to_string());
/// // Game { position: "4453", grid: 4210688, mask: 274743296, state: NonTerminal }
/// assert_eq!(game1, game2);
/// ```
//         grid                    mask
//  0  0  0  0  0  0  0     0  0  0  0  0  0  0
//  0  0  0  0  0  0  0     0  0  0  0  0  0  0
//  0  0  0  0  0  0  0     0  0  0  0  0  0  0
//  0  0  0  0  0  0  0     0  0  0  0  0  0  0
//  0  0  0  0  0  0  0     0  0  0  0  0  0  0
//  0  0  0  1  0  0  0     0  0  0  1  0  0  0
//  0  0  1  0  0  0  0     0  0  1  1  1  0  0
//
//         bits                   display
//  6 13 20 27 34 41 48     1  2  3  4  5  6  7
//  5 12 19 26 33 40 47     .  .  .  .  .  .  .
//  4 11 18 25 32 39 46     .  .  .  .  .  .  .
//  3 10 17 24 31 38 45     .  .  .  .  .  .  .
//  2  9 16 23 30 37 44     .  .  .  .  .  .  .
//  1  8 15 22 29 36 43     .  .  .  O  .  .  .
//  0  7 14 21 28 35 42     .  .  O  X  X  .  .
#[derive(Debug, PartialEq)]
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

    /// returns the score of the game's position
    /// where a position has
    /// - a positive score if the current player can win. 1 if they win with their last stone, 2 if
    ///   they win with their second to last stone, and so on...
    /// - a score of 0 if the game will end by a draw game
    /// - a negative score if the current player lose whatever they play. -1 if their opponent wins
    ///   with their last stone, -2 if their opponent wins with their second to last stone, and so
    ///   on...
    pub fn eval(&self) -> i8 { self.negamax() }

    /// returns new Ok(Game) where move_ was played
    pub fn play(&self, move_: &str) -> Result<Self> {
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

    fn get_state(grid: u64, mask: u64) -> State {
        for direction in [1, 6, 7, 8] {
            let intersected = grid << direction & grid;
            if intersected << 2 * direction & intersected != 0 { return Win; }
        }
        if mask & 279258638311359 == 279258638311359 { Draw } else { NonTerminal }
    }

    fn negamax(&self) -> i8 {
        match self.state {
            NonTerminal => {}
            Win => return (self.position.len() as i8 - 43) >> 1,
            Draw => return 0,
        }
        let mut max = i8::MIN + 1;
        for move_ in ["4", "3", "5", "2", "6", "1", "7"] {
            match self.play(move_) {
                Ok(game) => {
                    let score = -game.negamax();
                    if score > max { max = score; }
                }
                Err(_) => {}
            }
        }
        max
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
        write!(f, "\x1b[0m")?;
        Ok(())
    }
}
