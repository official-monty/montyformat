use crate::chess::{Castling, Move, Position};

pub struct SearchData {
    best_move: Move,
    score: f32,
    visit_distribution: Option<Vec<(Move, u32)>>,
}

impl SearchData {
    pub fn new<T: Copy + Into<Move>>(
        best_move: T,
        score: f32,
        visit_distribution: Option<Vec<(T, u32)>>,
    ) -> Self {
        Self{
            best_move: best_move.into(),
            score,
            visit_distribution: visit_distribution.map(|x| x.iter().map(|&(mov, visits)| (mov.into(), visits)).collect()),
        }
    }
}

pub struct MontyFormat {
    startpos: Position,
    castling: Castling,
    moves: Vec<SearchData>,
}

impl MontyFormat {
    pub fn new(startpos: Position, castling: Castling) -> Self {
        Self { startpos, castling, moves: Vec::new() }
    }

    pub fn push(&mut self, position_data: SearchData) {
        self.moves.push(position_data);
    }

    pub fn pop(&mut self) -> Option<SearchData> {
        self.moves.pop()
    }
}
