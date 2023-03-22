use once_cell::sync::Lazy;
use std::iter::once;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
        static CLOCKWISE_ROTATIONS: Lazy<[Piece; 19]> = Lazy::new(|| {
            let mut rotations = [Piece::None; 19];

            let pairs = Piece::get_rotation_cycles().iter().flat_map(|cycle| {
                cycle
                    .iter()
                    .zip(cycle.iter().skip(1).chain(once(cycle.first().unwrap())))
            });

            for (first, second) in pairs {
                rotations[*first as usize] = *second;
            }

            rotations
        });

        CLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub fn get_counterclockwise_rotation(self) -> Self {
        static COUNTERCLOCKWISE_ROTATIONS: Lazy<[Piece; 19]> = Lazy::new(|| {
            let mut rotations = [Piece::None; 19];

            let pairs = Piece::get_rotation_cycles().iter().flat_map(|cycle| {
                cycle
                    .iter()
                    .skip(1)
                    .chain(once(cycle.first().unwrap()))
                    .zip(cycle.iter())
            });

            for (first, second) in pairs {
                rotations[*first as usize] = *second;
            }

            rotations
        });

        COUNTERCLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub fn get_tile_offsets(self) -> &'static [(i8, i8); 4] {
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

    fn get_rotation_cycles() -> &'static [&'static [Piece]; 7] {
        static ROTATION_CYCLES: Lazy<[&[Piece]; 7]> = Lazy::new(|| {
            [
                [Piece::TUp, Piece::TRight, Piece::TDown, Piece::TLeft].as_slice(),
                [Piece::JUp, Piece::JRight, Piece::JDown, Piece::JLeft].as_slice(),
                [Piece::ZHorizontal, Piece::ZVertical].as_slice(),
                [Piece::O].as_slice(),
                [Piece::SHorizontal, Piece::SVertical].as_slice(),
                [Piece::LUp, Piece::LRight, Piece::LDown, Piece::LLeft].as_slice(),
                [Piece::IHorizontal, Piece::IVertical].as_slice(),
            ]
        });

        &ROTATION_CYCLES
    }
}
