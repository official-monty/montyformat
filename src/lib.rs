pub mod chess;

#[macro_export]
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

#[macro_export]
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