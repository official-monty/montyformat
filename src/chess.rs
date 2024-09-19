mod attacks;
mod consts;
mod frc;
mod moves;
mod position;

pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub use attacks::Attacks;
pub use consts::{Flag, Piece, Right, Side};
pub use frc::Castling;
pub use moves::Move;
pub use position::Position;

pub fn perft<const REPORT: bool>(pos: &Position, castling: &Castling, depth: u8) -> u64 {
    if depth == 1 {
        let mut count = 0;
        pos.map_legal_moves(castling, |_| count += 1);
        return count;
    }

    let mut count = 0;

    pos.map_legal_moves(castling, |mov| {
        let mut new = *pos;
        new.make(mov, castling);

        let sub_count = perft::<false>(&new, castling, depth - 1);

        if REPORT {
            println!("{}: {sub_count}", mov.to_uci(castling));
        }

        count += sub_count;
    });

    count
}
