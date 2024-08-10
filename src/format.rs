use std::io::{Error, ErrorKind, Write};

use crate::chess::{Castling, Move, Position};

pub struct SearchData {
    pub best_move: Move,
    pub score: f32,
    pub visit_distribution: Option<Vec<(Move, u32)>>,
}

impl SearchData {
    pub fn new<T: Copy + Into<Move>>(
        best_move: T,
        score: f32,
        visit_distribution: Option<Vec<(T, u32)>>,
    ) -> Self {
        let mut visit_distribution: Option<Vec<(Move, u32)>> = visit_distribution
            .map(|x| x.iter().map(|&(mov, visits)| (mov.into(), visits)).collect());

        if let Some(dist) = visit_distribution.as_mut() {
            dist.sort_by_key(|(mov, _)| mov.inner());
        }

        Self{
            best_move: best_move.into(),
            score,
            visit_distribution,
        }
    }
}

pub struct MontyFormat {
    pub startpos: Position,
    pub castling: Castling,
    pub result: f32,
    pub moves: Vec<SearchData>,
}

impl MontyFormat {
    pub fn new(startpos: Position, castling: Castling) -> Self {
        Self { startpos, castling, result: 0.0, moves: Vec::new() }
    }

    pub fn push(&mut self, position_data: SearchData) {
        self.moves.push(position_data);
    }

    pub fn pop(&mut self) -> Option<SearchData> {
        self.moves.pop()
    }

    #[must_use]
    pub fn serialise_into_buffer(&self, writer: &mut Vec<u8>) -> std::io::Result<()> {
        if !writer.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Buffer is not empty!"));
        }

        let compressed = CompressedChessBoard::from(self.startpos);

        for bb in compressed.bbs {
            writer.write_all(&bb.to_le_bytes())?;
        }

        writer.write_all(&compressed.stm.to_le_bytes())?;
        writer.write_all(&compressed.enp_sq.to_le_bytes())?;
        writer.write_all(&compressed.rights.to_le_bytes())?;
        writer.write_all(&compressed.halfm.to_le_bytes())?;
        writer.write_all(&compressed.fullm.to_le_bytes())?;

        for rf in self.castling.rook_files().as_flattened() {
            writer.write_all(&rf.to_le_bytes())?;
        }

        let result = (self.result * 2.0) as u8;
        writer.write_all(&result.to_le_bytes())?;

        for data in &self.moves {
            if data.score.clamp(0.0, 1.0) != data.score {
                return Err(Error::new(ErrorKind::InvalidData, "Score outside valid range!"));
            }

            let score = (data.score * f32::from(u16::MAX)) as u16;

            writer.write_all(&data.best_move.inner().to_le_bytes())?;
            writer.write_all(&score.to_le_bytes())?;

            let num_moves = data.visit_distribution.as_ref().map(|dist| dist.len()).unwrap_or(0) as u8;

            writer.write_all(&num_moves.to_le_bytes())?;

            if let Some(dist) = data.visit_distribution.as_ref() {
                let max_visits = dist.iter().max_by_key(|(_, visits)| visits).map(|x| x.1).unwrap_or(0);
                for (_, visits) in dist {
                    let scaled_visits = (*visits as f32 * 256.0 / max_visits as f32) as u16;
                    writer.write_all(&scaled_visits.to_le_bytes())?;
                }
            }
        }

        writer.write_all(&[0; 2])?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct CompressedChessBoard {
    bbs: [u64; 4],
    stm: u8,
    enp_sq: u8,
    rights: u8,
    halfm: u8,
    fullm: u16,
}

impl From<Position> for CompressedChessBoard {
    fn from(board: Position) -> Self {
        let bbs = board.bbs();

        Self {
            bbs: [
                bbs[1],
                bbs[5] ^ bbs[6] ^ bbs[7],
                bbs[3] ^ bbs[4] ^ bbs[7],
                bbs[2] ^ bbs[4] ^ bbs[6],
            ],
            stm: board.stm() as u8,
            enp_sq: board.enp_sq(),
            rights: board.rights(),
            halfm: board.halfm(),
            fullm: board.fullm(),
        }
    }
}

impl From<CompressedChessBoard> for Position {
    fn from(value: CompressedChessBoard) -> Self {
        let qbbs = value.bbs;

        let mut bbs = [0; 8];

        let blc = qbbs[0];
        let rqk = qbbs[1];
        let nbk = qbbs[2];
        let pbq = qbbs[3];

        let occ = rqk | nbk | pbq;
        let pnb = occ ^ qbbs[1];
        let prq = occ ^ qbbs[2];
        let nrk = occ ^ qbbs[3];

        bbs[0] = occ ^ blc;
        bbs[1] = blc;
        bbs[2] = pnb & prq;
        bbs[3] = pnb & nrk;
        bbs[4] = pnb & nbk & pbq;
        bbs[5] = prq & nrk;
        bbs[6] = pbq & prq & rqk;
        bbs[7] = nbk & rqk;

        Position::from_raw(bbs, value.stm > 0, value.enp_sq, value.rights, value.halfm, value.fullm)
    }
}
