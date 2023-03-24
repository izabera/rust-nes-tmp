#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    A,
    B,
    C,
}

impl From<Piece> for Tile {
    fn from(piece: Piece) -> Self {
        match piece as usize {
            0..=3 | 10 | 17 | 18 => Tile::A,
            4..=7 | 11 | 12 => Tile::B,
            8 | 9 | 13..=16 => Tile::C,
            19 => Tile::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    TUp = 0,
    TRight = 1,
    TDown = 2,
    TLeft = 3,
    JUp = 4,
    JRight = 5,
    JDown = 6,
    JLeft = 7,
    ZHorizontal = 8,
    ZVertical = 9,
    O = 10,
    SHorizontal = 11,
    SVertical = 12,
    LUp = 13,
    LRight = 14,
    LDown = 15,
    LLeft = 16,
    IVertical = 17,
    IHorizontal = 18,
    None = 19,
}


impl Piece {
    #[must_use]
    pub fn get_clockwise_rotation(self) -> Self {
        use Piece::*;
        const CLOCKWISE_ROTATIONS: [Piece; 19] = [
            TRight, TDown, TLeft, TUp, JRight, JDown, JLeft, JUp, ZVertical, ZHorizontal, O, SVertical, SHorizontal, LRight, LDown, LLeft, LUp, IHorizontal, IVertical
        ];
        CLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub fn get_counterclockwise_rotation(self) -> Self {
        use Piece::*;
        const COUNTERCLOCKWISE_ROTATIONS: [Piece; 19] = [
            TLeft, TUp, TRight, TDown, JLeft, JUp, JRight, JDown, ZVertical, ZHorizontal, O, SVertical, SHorizontal, LLeft, LUp, LRight, LDown, IHorizontal, IVertical
        ];

        COUNTERCLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub const fn get_tile_offsets(self) -> &'static [(i8, i8); 4] {
        const TILE_OFFSETS: [[(i8, i8); 4]; 19] = [
            [(-1, 0), (0, 0), (1, 0), (0, -1)],
            [(0, -1), (0, 0), (1, 0), (0, 1)],
            [(-1, 0), (0, 0), (1, 0), (0, 1)],
            [(0, -1), (-1, 0), (0, 0), (0, 1)],
            [(0, -1), (0, 0), (-1, 1), (0, 1)],
            [(-1, -1), (-1, 0), (0, 0), (1, 0)],
            [(0, -1), (1, -1), (0, 0), (0, 1)],
            [(-1, 0), (0, 0), (1, 0), (1, 1)],
            [(-1, 0), (0, 0), (0, 1), (1, 1)],
            [(1, -1), (0, 0), (1, 0), (0, 1)],
            [(-1, 0), (0, 0), (-1, 1), (0, 1)],
            [(0, 0), (1, 0), (-1, 1), (0, 1)],
            [(0, -1), (0, 0), (1, 0), (1, 1)],
            [(0, -1), (0, 0), (0, 1), (1, 1)],
            [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            [(-1, -1), (0, -1), (0, 0), (0, 1)],
            [(1, -1), (-1, 0), (0, 0), (1, 0)],
            [(0, -2), (0, -1), (0, 0), (0, 1)],
            [(-2, 0), (-1, 0), (0, 0), (1, 0)],
        ];

        &TILE_OFFSETS[self as usize]
    }
}
