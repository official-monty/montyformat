pub mod chess;
mod format;

pub use format::{MontyFormat, SearchData};

macro_rules! init {
    (|$sq:ident, $size:literal | $($rest:tt)+) => {{
        let mut $sq = 0;
        let mut res = [{$($rest)+}; $size];
        while $sq < $size {
            res[$sq] = {$($rest)+};
            $sq += 1;
        }
        res
    }};
}

macro_rules! bitloop {
    (| $bb:expr, $sq:ident | $func:expr) => {{
        let mut bb = $bb;

        while bb > 0 {
            let $sq = bb.trailing_zeros() as u16;
            bb &= bb - 1;

            $func;
        }
    };}
}

macro_rules! read_into_primitive {
    ($reader:expr, $t:ty) => {{
        let mut buf = [0u8; std::mem::size_of::<$t>()];
        $reader.read_exact(&mut buf)?;
        <$t>::from_le_bytes(buf)
    }};
}

pub(crate) use init;
pub(crate) use bitloop;
pub(crate) use read_into_primitive;
