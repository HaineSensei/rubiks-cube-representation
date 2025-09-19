// Face turn operations will go here when implemented

/// Standard single-layer face turns (U, D, L, R, F, B)
pub enum BasicMove {
    U,
    U2,
    U3,
    D,
    D2,
    D3,
    L,
    L2,
    L3,
    R,
    R2,
    R3,
    F,
    F2,
    F3,
    B,
    B2,
    B3
}
pub use BasicMove::*;

/// Multi-layer wide turns with depth parameter (e.g., Uw(2) for 2-layer wide U)
pub enum WideMove {
    Uw(usize),
    Uw2(usize),
    Uw3(usize),
    Dw(usize),
    Dw2(usize),
    Dw3(usize),
    Lw(usize),
    Lw2(usize),
    Lw3(usize),
    Rw(usize),
    Rw2(usize),
    Rw3(usize),
    Fw(usize),
    Fw2(usize),
    Fw3(usize),
    Bw(usize),
    Bw2(usize),
    Bw3(usize)
}
pub use WideMove::*;

/// Individual slice turns by layer number (e.g., Us(2) for slice 2)
pub enum SliceMove {
    Us(usize),
    Us2(usize),
    Us3(usize),
    Ds(usize),
    Ds2(usize),
    Ds3(usize),
    Ls(usize),
    Ls2(usize),
    Ls3(usize),
    Rs(usize),
    Rs2(usize),
    Rs3(usize),
    Fs(usize),
    Fs2(usize),
    Fs3(usize),
    Bs(usize),
    Bs2(usize),
    Bs3(usize)
}
pub use SliceMove::*;

/// Range of layer turns (e.g., Ur(2,4) for layers 2 through 4)
pub enum RangeMove {
    Ur(usize,usize),
    Ur2(usize,usize),
    Ur3(usize,usize),
    Dr(usize,usize),
    Dr2(usize,usize),
    Dr3(usize,usize),
    Lr(usize,usize),
    Lr2(usize,usize),
    Lr3(usize,usize),
    Rr(usize,usize),
    Rr2(usize,usize),
    Rr3(usize,usize),
    Fr(usize,usize),
    Fr2(usize,usize),
    Fr3(usize,usize),
    Br(usize,usize),
    Br2(usize,usize),
    Br3(usize,usize)
}
pub use RangeMove::*;

/*
from Cubelelo:
M (Middle) – The layer between L and R, moves like an L turn.
E (Equator) – The layer between U and D, moves like a D turn.
S (Standing) – The layer between F and B, moves like an F turn. 
*/
/// Traditional middle slice moves for odd-dimensioned cubes
pub enum MiddleMove {
    //middle
    M,
    M2,
    M3,
    //equator
    E,
    E2,
    E3,
    //standing
    S,
    S2,
    S3,
}
pub use MiddleMove::*;
