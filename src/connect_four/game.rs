use crate::prelude::*;
use std::fmt::{Display, Formatter};

/// ```
/// use connect_four::prelude::*;
///
/// let game1 = Game::new("445").play(2).unwrap(); // column in play is 0-indexed
/// let game2 = Game::new("4453");
/// // Game { moves: 4, grid: 4210688, mask: 274743296, state: NonTerminal }
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
    moves: usize,
    grid: u64,
    mask: u64,
    state: State,
}

impl Game {
    pub fn new(position: &str) -> Self {
        let mut game = Game {
            moves: 0,
            grid: 0,
            mask: 0,
            state: NonTerminal,
        };

        if position.len() == 0 { return game; }

        for ch in position.chars() {
            let column = ch.to_string().parse::<usize>().unwrap() - 1;
            game = game.play(column).unwrap();
        }
        game
    }

    /// returns the score of the game's position
    /// where a position has
    /// - a positive score if the current player can win. 1 if they win with their last stone, 2 if
    ///   they win with their second to last stone, and so on...
    /// - a score of 0 if the game will end by a draw game
    /// - a negative score if the current player lose whatever they play. -1 if their opponent wins
    ///   with their last stone, -2 if their opponent wins with their second to last stone, and so
    ///   on...
    pub fn eval(&self) -> i8 { self.negamax(i8::MIN + 1, i8::MAX) }

    /// returns new Ok(Game) where move_ was played
    pub fn play(&self, column: usize) -> Result<Self> {
        match self.state {
            NonTerminal => {}
            Win | Draw => { return Err("self.state is terminal; game has concluded".into()) }
        }
        if column > 6 { return Err(ValueError(format!("play(column: usize) expects column < 7; got {}", column))); }
        let x = 1 << 7 * column;
        let no_go = x << 5;
        if self.mask & no_go == no_go { return Err(format!("column {} already filled", column).into()); }

        let moves = self.moves + 1;
        let mask = self.mask + x | self.mask;
        let grid = self.grid ^ mask;
        let state = Self::get_state(grid, mask);

        let grid = Game {
            moves,
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

    fn negamax(&self, mut alpha: i8, mut beta: i8) -> i8 {
        match self.state {
            NonTerminal => {}
            Win => return (self.moves as i8 - 43) >> 1,
            Draw => return 0,
        }

        let max = ((41 - self.moves as i8) >> 1) + 1;
        if beta > max {
            beta = max;
            if alpha >= beta { return beta; }
        }

        for column in [3, 2, 4, 5, 1, 0, 6] {
            match self.play(column) {
                Ok(game) => {
                    let score = -game.negamax(-beta, -alpha);
                    if score >= beta { return score; }
                    if score > alpha { alpha = score; }
                }
                Err(_) => {}
            }
        }
        alpha
    }

    /// returns Player to play
    fn to_play(&self) -> Player { (self.moves).into() }
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
