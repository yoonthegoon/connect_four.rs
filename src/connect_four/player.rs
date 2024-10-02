#[derive(Debug)]
pub enum Player {
    Player1,
    Player2,
}

impl From<usize> for Player {
    fn from(n: usize) -> Self {
        match n % 2 {
            0 => Player::Player1,
            _ => Player::Player2,
        }
    }
}

impl From<u64> for Player { fn from(n: u64) -> Self { Self::from(n as usize) } }
