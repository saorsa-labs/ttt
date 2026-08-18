//! Local 3×3 tic-tac-toe. Human is X, computer is O.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win(Player),
    Draw,
}

#[derive(Clone, Debug)]
pub struct Game {
    cells: [Cell; 9],
    turn: Player,
    outcome: Option<Outcome>,
    winning_line: Option<[usize; 3]>,
}

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
            turn: Player::X,
            outcome: None,
            winning_line: None,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn cell(&self, index: usize) -> Cell {
        self.cells[index]
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    pub fn winning_line(&self) -> Option<[usize; 3]> {
        self.winning_line
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    /// Human (X) move, then computer (O) replies if the game is still on.
    pub fn play_human(&mut self, index: usize) -> bool {
        if self.turn != Player::X {
            return false;
        }
        if !self.play(index) {
            return false;
        }
        self.play_computer();
        true
    }

    pub fn play_at(&mut self, row: usize, col: usize) -> bool {
        if row > 2 || col > 2 {
            return false;
        }
        self.play_human(row * 3 + col)
    }

    pub fn play_computer(&mut self) {
        if self.is_over() || self.turn != Player::O {
            return;
        }
        if let Some(index) = self.best_move() {
            self.play(index);
        }
    }

    /// Play at `index` 0..=8. Returns false if ignored (occupied or game over).
    pub fn play(&mut self, index: usize) -> bool {
        if self.is_over() || index >= 9 || self.cells[index] != Cell::Empty {
            return false;
        }
        self.cells[index] = Cell::Occupied(self.turn);
        if let Some(line) = LINES.iter().find(|line| {
            line.iter()
                .all(|&i| self.cells[i] == Cell::Occupied(self.turn))
        }) {
            self.winning_line = Some(*line);
            self.outcome = Some(Outcome::Win(self.turn));
        } else if self.cells.iter().all(|c| *c != Cell::Empty) {
            self.outcome = Some(Outcome::Draw);
        } else {
            self.turn = self.turn.other();
        }
        true
    }

    fn best_move(&self) -> Option<usize> {
        let mut best_i = None;
        let mut best_score = i32::MIN;
        for i in 0..9 {
            if self.cells[i] != Cell::Empty {
                continue;
            }
            let mut next = self.clone();
            next.play(i);
            let score = next.minimax();
            if score > best_score {
                best_score = score;
                best_i = Some(i);
            }
        }
        best_i
    }

    fn minimax(&self) -> i32 {
        match self.outcome {
            Some(Outcome::Win(Player::O)) => 1,
            Some(Outcome::Win(Player::X)) => -1,
            Some(Outcome::Draw) => 0,
            None => {
                if self.turn == Player::O {
                    let mut best = i32::MIN;
                    for i in 0..9 {
                        if self.cells[i] == Cell::Empty {
                            let mut next = self.clone();
                            next.play(i);
                            best = best.max(next.minimax());
                        }
                    }
                    best
                } else {
                    let mut best = i32::MAX;
                    for i in 0..9 {
                        if self.cells[i] == Cell::Empty {
                            let mut next = self.clone();
                            next.play(i);
                            best = best.min(next.minimax());
                        }
                    }
                    best
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_starts() {
        let g = Game::new();
        assert_eq!(g.turn(), Player::X);
        assert!(!g.is_over());
    }

    #[test]
    fn x_wins_top_row() {
        let mut g = Game::new();
        assert!(g.play(0));
        assert!(g.play(3));
        assert!(g.play(1));
        assert!(g.play(4));
        assert!(g.play(2));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::X)));
        assert_eq!(g.winning_line(), Some([0, 1, 2]));
        assert!(!g.play(5));
    }

    #[test]
    fn draw() {
        let mut g = Game::new();
        for i in [0, 1, 2, 5, 3, 6, 4, 8, 7] {
            assert!(g.play(i));
        }
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn computer_blocks_immediate_win() {
        let mut g = Game::new();
        assert!(g.play(0)); // X
        assert!(g.play(4)); // O center
        assert!(g.play(1)); // X
        g.play_computer();
        assert_eq!(g.cell(2), Cell::Occupied(Player::O));
    }

    #[test]
    fn computer_never_loses_from_start() {
        // Exhaust first-move replies: after X plays each opening, perfect O
        // plus perfect X thereafter is never a computer loss.
        for opening in 0..9 {
            let mut g = Game::new();
            assert!(g.play_human(opening));
            while !g.is_over() && g.turn() == Player::X {
                let mut played = false;
                for i in 0..9 {
                    if g.cell(i) == Cell::Empty {
                        let mut trial = g.clone();
                        trial.play(i);
                        if trial.minimax() <= 0 {
                            g.play_human(i);
                            played = true;
                            break;
                        }
                    }
                }
                if !played {
                    // X has no non-losing move; take first empty
                    for i in 0..9 {
                        if g.cell(i) == Cell::Empty {
                            g.play_human(i);
                            break;
                        }
                    }
                }
            }
            assert_ne!(g.outcome(), Some(Outcome::Win(Player::X)));
        }
    }
}
