// Face turn operations will go here when implemented

pub enum BasicMove {
    U,
    D,
    L,
    R,
    F,
    B
}
pub use BasicMove::*;

pub enum WideMove {
    Uw(usize),
    Dw(usize),
    Lw(usize),
    Rw(usize),
    Fw(usize),
    Bw(usize)
}
pub use WideMove::*;

pub enum SliceMove {
    Us(usize),
    Ds(usize),
    Ls(usize),
    Rs(usize),
    Fs(usize),
    Bs(usize)
}
pub use SliceMove::*;

pub enum RangeMove {
    Ur(usize,usize),
    Dr(usize,usize),
    Lr(usize,usize),
    Rr(usize,usize),
    Fr(usize,usize),
    Br(usize,usize)
}
pub use RangeMove::*;

/*
from Cubelelo:
M (Middle) – The layer between L and R, moves like an L turn.
E (Equator) – The layer between U and D, moves like a D turn.
S (Standing) – The layer between F and B, moves like an F turn. 
*/
pub enum MiddleMove {
    //middle
    M,
    //equator
    E,
    //standing
    S,
}
pub use MiddleMove::*;
