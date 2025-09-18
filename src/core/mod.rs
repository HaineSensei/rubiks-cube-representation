pub mod cube;
pub mod rubiks;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

pub const COLOURS: [Colour;6] = [Colour::White, Colour::Yellow, Colour::Red, Colour::Orange, Colour::Blue, Colour::Green];