//! Cubie, slot, and orientation conventions.
//!
//! The variants of `EdgeCubie` and `CornerCubie` are declared in slot order:
//! variant zero belongs in slot zero in a solved cube, variant one in slot one,
//! and so on. The colors within each variant are also ordered.
//!
//! Orientation is measured relative to the ordered colors of the destination
//! slot. Orientation zero means that the cubie's first color points in the same
//! direction as the slot's first color. For an edge, orientation one swaps the
//! two directions. For a corner, orientation one is one clockwise twist and
//! orientation two is two clockwise twists (equivalently, one counterclockwise
//! twist), viewed from outside the cube looking directly at that corner.
//!
//! Piece Numbers:
//!          | 0| 0| 1|
//!          | 3| W| 1|
//!          | 3| 2| 2|
//! |  |  |  |  |  |  |  |  |  |
//! |  |  |  | 7| G| 6|  |  |  |
//! |  |  |  |  |  |  |  |  |  |
//!          | 4| 8| 5|
//!          |11| Y| 9|
//!          | 7|10| 6|
//!          |  |  |  |
//!          | 4| B| 5|
//!          |  |  |  |
use rand::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CubeError {
    InvalidEdgeSlot { value: u8 },
    InvalidCornerSlot { value: u8 },
    InvalidCube,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StickerColor {
    White,
    Yellow,
    Blue,
    Green,
    Red,
    Orange,
}

impl StickerColor {
    pub const ALL: [Self; 6] = [
        Self::White,
        Self::Yellow,
        Self::Blue,
        Self::Green,
        Self::Red,
        Self::Orange,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EdgeCubie {
    WB,
    WR,
    WG,
    WO,
    BO,
    BR,
    GR,
    GO,
    YG,
    YR,
    YB,
    YO,
}

impl EdgeCubie {
    fn get_sticker_colors(&self) -> [StickerColor; 2] {
        match *self {
            Self::WB => [StickerColor::White, StickerColor::Blue],
            Self::WR => [StickerColor::White, StickerColor::Red],
            Self::WG => [StickerColor::White, StickerColor::Green],
            Self::WO => [StickerColor::White, StickerColor::Orange],
            Self::BO => [StickerColor::Blue, StickerColor::Orange],
            Self::BR => [StickerColor::Blue, StickerColor::Red],
            Self::GR => [StickerColor::Green, StickerColor::Red],
            Self::GO => [StickerColor::Green, StickerColor::Orange],
            Self::YB => [StickerColor::Yellow, StickerColor::Blue],
            Self::YO => [StickerColor::Yellow, StickerColor::Orange],
            Self::YG => [StickerColor::Yellow, StickerColor::Green],
            Self::YR => [StickerColor::Yellow, StickerColor::Red],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EdgeOrientation(u8);

impl EdgeOrientation {
    fn new(value: u8) -> Self {
        Self(value % 2)
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub fn is(self, orientation: u8) -> bool {
        self.0 == orientation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeSlot(u8);

impl EdgeSlot {
    fn new(value: u8) -> Result<Self, CubeError> {
        if value < 12 {
            Ok(Self(value))
        } else {
            Err(CubeError::InvalidEdgeSlot { value })
        }
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub fn as_idx(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EdgePiece {
    cubie: EdgeCubie,
    orientation: EdgeOrientation,
}

impl EdgePiece {
    pub fn new(cubie: EdgeCubie, orientation: u8) -> Self {
        Self {
            cubie,
            orientation: EdgeOrientation::new(orientation),
        }
    }
    pub fn cubie(&self) -> &EdgeCubie {
        &self.cubie
    }

    pub fn orientation(&self) -> &EdgeOrientation {
        &self.orientation
    }

    pub fn get_sticker_colors(&self) -> [StickerColor; 2] {
        let mut sticker_colors = self.cubie.get_sticker_colors();
        sticker_colors.rotate_right(self.orientation.as_u8() as usize);
        sticker_colors
    }

    pub fn with_twist(&mut self, amount: u8) -> Self {
        self.orientation = EdgeOrientation::new(self.orientation.as_u8() + amount);
        *self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CornerCubie {
    WOB,
    WBR,
    WRG,
    WGO,
    YOG,
    YGR,
    YRB,
    YBO,
}

impl CornerCubie {
    #[rustfmt::skip]
    fn get_sticker_colors(&self) -> [StickerColor; 3] {
        match *self {
            Self::WOB => [
                StickerColor::White,
                StickerColor::Orange,
                StickerColor::Blue,
            ],
            Self::WBR => [
                StickerColor::White,
                StickerColor::Blue,
                StickerColor::Red
            ],
            Self::WRG => [
                StickerColor::White,
                StickerColor::Red,
                StickerColor::Green
            ],
            Self::WGO => [
                StickerColor::White,
                StickerColor::Green,
                StickerColor::Orange,
            ],
            Self::YRB => [
                StickerColor::Yellow,
                StickerColor::Red,
                StickerColor::Blue
            ],
            Self::YBO => [
                StickerColor::Yellow,
                StickerColor::Blue,
                StickerColor::Orange,
            ],
            Self::YOG => [
                StickerColor::Yellow,
                StickerColor::Orange,
                StickerColor::Green,
            ],
            Self::YGR => [
                StickerColor::Yellow,
                StickerColor::Green,
                StickerColor::Red
            ],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CornerOrientation(u8);

impl CornerOrientation {
    fn new(value: u8) -> Self {
        CornerOrientation(value % 3)
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub fn is(self, orientation: u8) -> bool {
        self.0 == orientation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CornerSlot(u8);

impl CornerSlot {
    fn new(value: u8) -> Result<Self, CubeError> {
        if value < 8 {
            Ok(Self(value))
        } else {
            Err(CubeError::InvalidCornerSlot { value })
        }
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
    pub fn as_idx(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CornerPiece {
    cubie: CornerCubie,
    orientation: CornerOrientation,
}

impl CornerPiece {
    pub fn new(cubie: CornerCubie, orientation: u8) -> Self {
        Self {
            cubie,
            orientation: CornerOrientation::new(orientation),
        }
    }

    pub fn cubie(&self) -> &CornerCubie {
        &self.cubie
    }

    pub fn orientation(&self) -> &CornerOrientation {
        &self.orientation
    }

    pub fn get_sticker_colors(&self) -> [StickerColor; 3] {
        let mut sticker_colors = self.cubie.get_sticker_colors();
        sticker_colors.rotate_right(self.orientation.as_u8() as usize);
        sticker_colors
    }

    pub fn with_twist(&mut self, amount: u8) -> Self {
        self.orientation = CornerOrientation::new(self.orientation.as_u8() + amount);
        *self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CubeMove {
    U,
    UPrime,
    U2,
    D,
    DPrime,
    D2,
    B,
    BPrime,
    B2,
    F,
    FPrime,
    F2,
    R,
    RPrime,
    R2,
    L,
    LPrime,
    L2,
}

impl CubeMove {
    pub const ALL: [CubeMove; 18] = [
        CubeMove::U,
        CubeMove::UPrime,
        CubeMove::U2,
        CubeMove::D,
        CubeMove::DPrime,
        CubeMove::D2,
        CubeMove::B,
        CubeMove::BPrime,
        CubeMove::B2,
        CubeMove::F,
        CubeMove::FPrime,
        CubeMove::F2,
        CubeMove::R,
        CubeMove::RPrime,
        CubeMove::R2,
        CubeMove::L,
        CubeMove::LPrime,
        CubeMove::L2,
    ];
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Cube {
    corners: [CornerPiece; 8],
    edges: [EdgePiece; 12],
}

impl Cube {
    pub fn new(corners: [CornerPiece; 8], edges: [EdgePiece; 12]) -> Result<Self, CubeError> {
        let cube = Self { corners, edges };
        if cube.edge_parity() == 0
            && cube.corner_parity() == 0
            && cube.all_different_corners()
            && cube.all_different_edges()
        {
            return Ok(cube);
        }
        Err(CubeError::InvalidCube)
    }

    pub fn solved() -> Self {
        Self {
            edges: [
                EdgePiece {
                    cubie: EdgeCubie::WB,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::WR,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::WG,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::WO,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::BO,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::BR,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::GR,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::GO,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::YG,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::YR,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::YB,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::YO,
                    orientation: EdgeOrientation::new(0),
                },
            ],
            corners: [
                CornerPiece {
                    cubie: CornerCubie::WOB,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::WBR,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::WRG,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::WGO,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::YOG,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::YGR,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::YRB,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::YBO,
                    orientation: CornerOrientation::new(0),
                },
            ],
        }
    }

    pub fn make_solved(&mut self) {
        *self = Self::solved();
    }

    pub fn generate_scramble(num_moves: usize) -> Vec<CubeMove> {
        let mut rng = rand::rng();
        let mut scramble_moves: Vec<CubeMove> = Vec::new();
        for _ in 0..num_moves {
            scramble_moves.push(*CubeMove::ALL.choose(&mut rng).unwrap());
        }
        scramble_moves
    }

    pub fn apply_scramble(&mut self, scramble_moves: Vec<CubeMove>) {
        for scramble_move in scramble_moves {
            self.apply_move(scramble_move);
        }
    }

    pub fn corners(&self) -> &[CornerPiece; 8] {
        &self.corners
    }

    pub fn edges(&self) -> &[EdgePiece; 12] {
        &self.edges
    }

    fn edge_parity(&self) -> u8 {
        let mut edge_parity = 0;
        for edge in &self.edges {
            edge_parity = (edge_parity + edge.orientation.as_u8()) % 2;
        }
        edge_parity
    }

    fn corner_parity(&self) -> u8 {
        let mut corner_parity = 0;
        for corner in &self.corners {
            corner_parity = (corner_parity + corner.orientation.as_u8()) % 3;
        }
        corner_parity
    }

    fn all_different_edges(&self) -> bool {
        let mut seen = Vec::<EdgeCubie>::new();
        for edge in &self.edges {
            let cubie = &edge.cubie;
            if seen.contains(cubie) {
                return false;
            }
            seen.push(*cubie);
        }
        true
    }

    fn all_different_corners(&self) -> bool {
        let mut seen = Vec::<CornerCubie>::new();
        for corner in &self.corners {
            let cubie = &corner.cubie;
            if seen.contains(cubie) {
                return false;
            }
            seen.push(*cubie);
        }
        true
    }

    fn is_solved(&self) -> bool {
        self == &Cube::solved()
    }

    pub fn apply_move(&mut self, cube_move: CubeMove) {
        match cube_move {
            CubeMove::U => self.move_u(),
            CubeMove::UPrime => self.move_u_prime(),
            CubeMove::U2 => self.move_u2(),
            CubeMove::D => self.move_d(),
            CubeMove::DPrime => self.move_d_prime(),
            CubeMove::D2 => self.move_d2(),
            CubeMove::B => self.move_b(),
            CubeMove::BPrime => self.move_b_prime(),
            CubeMove::B2 => self.move_b2(),
            CubeMove::F => self.move_f(),
            CubeMove::FPrime => self.move_f_prime(),
            CubeMove::F2 => self.move_f2(),
            CubeMove::R => self.move_r(),
            CubeMove::RPrime => self.move_r_prime(),
            CubeMove::R2 => self.move_r2(),
            CubeMove::L => self.move_l(),
            CubeMove::LPrime => self.move_l_prime(),
            CubeMove::L2 => self.move_l2(),
            _ => (),
        }
    }

    pub fn apply_moves(&mut self, cube_moves: Vec<CubeMove>) {
        for cube_move in cube_moves {
            self.apply_move(cube_move);
        }
    }

    fn move_u(&mut self) {
        let tmp_corner = self.corners[0];
        self.corners[0] = self.corners[3];
        self.corners[3] = self.corners[2];
        self.corners[2] = self.corners[1];
        self.corners[1] = tmp_corner;

        let tmp_edge = self.edges[0];
        self.edges[0] = self.edges[3];
        self.edges[3] = self.edges[2];
        self.edges[2] = self.edges[1];
        self.edges[1] = tmp_edge;
    }

    fn move_u_prime(&mut self) {
        let tmp_corner = self.corners[0];
        self.corners[0] = self.corners[1];
        self.corners[1] = self.corners[2];
        self.corners[2] = self.corners[3];
        self.corners[3] = tmp_corner;

        let tmp_edge = self.edges[0];
        self.edges[0] = self.edges[1];
        self.edges[1] = self.edges[2];
        self.edges[2] = self.edges[3];
        self.edges[3] = tmp_edge;
    }

    fn move_u2(&mut self) {
        self.corners.swap(0, 2);
        self.corners.swap(1, 3);

        self.edges.swap(0, 2);
        self.edges.swap(1, 3);
    }

    fn move_d(&mut self) {
        let tmp_corner = self.corners[4];
        self.corners[4] = self.corners[7];
        self.corners[7] = self.corners[6];
        self.corners[6] = self.corners[5];
        self.corners[5] = tmp_corner;

        let tmp_edge = self.edges[8];
        self.edges[8] = self.edges[11];
        self.edges[11] = self.edges[10];
        self.edges[10] = self.edges[9];
        self.edges[9] = tmp_edge;
    }

    fn move_d_prime(&mut self) {
        let tmp_corner = self.corners[4];
        self.corners[4] = self.corners[5];
        self.corners[5] = self.corners[6];
        self.corners[6] = self.corners[7];
        self.corners[7] = tmp_corner;

        let tmp_edge = self.edges[8];
        self.edges[8] = self.edges[9];
        self.edges[9] = self.edges[10];
        self.edges[10] = self.edges[11];
        self.edges[11] = tmp_edge;
    }

    fn move_d2(&mut self) {
        self.corners.swap(4, 6);
        self.corners.swap(5, 7);

        self.edges.swap(8, 10);
        self.edges.swap(9, 11);
    }

    fn move_b(&mut self) {
        let mut tmp_corner = self.corners[0];
        self.corners[0] = self.corners[1].with_twist(1);
        self.corners[1] = self.corners[6].with_twist(2);
        self.corners[6] = self.corners[7].with_twist(1);
        self.corners[7] = tmp_corner.with_twist(2);

        let mut tmp_edge = self.edges[0];
        self.edges[0] = self.edges[5].with_twist(1);
        self.edges[5] = self.edges[10].with_twist(1);
        self.edges[10] = self.edges[4].with_twist(1);
        self.edges[4] = tmp_edge.with_twist(1);
    }

    fn move_b_prime(&mut self) {
        let mut tmp_corner = self.corners[0];
        self.corners[0] = self.corners[7].with_twist(1);
        self.corners[7] = self.corners[6].with_twist(2);
        self.corners[6] = self.corners[1].with_twist(1);
        self.corners[1] = tmp_corner.with_twist(2);

        let mut tmp_edge = self.edges[0];
        self.edges[0] = self.edges[4].with_twist(1);
        self.edges[4] = self.edges[10].with_twist(1);
        self.edges[10] = self.edges[5].with_twist(1);
        self.edges[5] = tmp_edge.with_twist(1);
    }

    fn move_b2(&mut self) {
        self.corners.swap(0, 6);
        self.corners.swap(1, 7);

        self.edges.swap(0, 10);
        self.edges.swap(4, 5);
    }

    fn move_f(&mut self) {
        let mut tmp_corner = self.corners[3];
        self.corners[3] = self.corners[4].with_twist(2);
        self.corners[4] = self.corners[5].with_twist(1);
        self.corners[5] = self.corners[2].with_twist(2);
        self.corners[2] = tmp_corner.with_twist(1);

        let mut tmp_edge = self.edges[2];
        self.edges[2] = self.edges[7].with_twist(1);
        self.edges[7] = self.edges[8].with_twist(1);
        self.edges[8] = self.edges[6].with_twist(1);
        self.edges[6] = tmp_edge.with_twist(1);
    }

    fn move_f_prime(&mut self) {
        let mut tmp_corner = self.corners[3];
        self.corners[3] = self.corners[2].with_twist(2);
        self.corners[2] = self.corners[5].with_twist(1);
        self.corners[5] = self.corners[4].with_twist(2);
        self.corners[4] = tmp_corner.with_twist(1);

        let mut tmp_edge = self.edges[2];
        self.edges[2] = self.edges[6].with_twist(1);
        self.edges[6] = self.edges[8].with_twist(1);
        self.edges[8] = self.edges[7].with_twist(1);
        self.edges[7] = tmp_edge.with_twist(1);
    }

    fn move_f2(&mut self) {
        self.corners.swap(3, 5);
        self.corners.swap(2, 4);

        self.edges.swap(2, 8);
        self.edges.swap(6, 7);
    }

    fn move_r(&mut self) {
        let mut tmp_corner = self.corners[1];
        self.corners[1] = self.corners[2].with_twist(1);
        self.corners[2] = self.corners[5].with_twist(2);
        self.corners[5] = self.corners[6].with_twist(1);
        self.corners[6] = tmp_corner.with_twist(2);

        let tmp_edge = self.edges[1];
        self.edges[1] = self.edges[6];
        self.edges[6] = self.edges[9];
        self.edges[9] = self.edges[5];
        self.edges[5] = tmp_edge;
    }

    fn move_r_prime(&mut self) {
        let mut tmp_corner = self.corners[1];
        self.corners[1] = self.corners[6].with_twist(1);
        self.corners[6] = self.corners[5].with_twist(2);
        self.corners[5] = self.corners[2].with_twist(1);
        self.corners[2] = tmp_corner.with_twist(2);

        let tmp_edge = self.edges[1];
        self.edges[1] = self.edges[5];
        self.edges[5] = self.edges[9];
        self.edges[9] = self.edges[6];
        self.edges[6] = tmp_edge;
    }

    fn move_r2(&mut self) {
        self.corners.swap(1, 5);
        self.corners.swap(6, 2);

        self.edges.swap(1, 9);
        self.edges.swap(5, 6);
    }

    fn move_l(&mut self) {
        let mut tmp_corner = self.corners[0];
        self.corners[0] = self.corners[7].with_twist(2);
        self.corners[7] = self.corners[4].with_twist(1);
        self.corners[4] = self.corners[3].with_twist(2);
        self.corners[3] = tmp_corner.with_twist(1);

        let tmp_edge = self.edges[3];
        self.edges[3] = self.edges[4];
        self.edges[4] = self.edges[11];
        self.edges[11] = self.edges[7];
        self.edges[7] = tmp_edge;
    }

    fn move_l_prime(&mut self) {
        let mut tmp_corner = self.corners[0];
        self.corners[0] = self.corners[3].with_twist(2);
        self.corners[3] = self.corners[4].with_twist(1);
        self.corners[4] = self.corners[7].with_twist(2);
        self.corners[7] = tmp_corner.with_twist(1);

        let tmp_edge = self.edges[3];
        self.edges[3] = self.edges[7];
        self.edges[7] = self.edges[11];
        self.edges[11] = self.edges[4];
        self.edges[4] = tmp_edge;
    }

    fn move_l2(&mut self) {
        self.corners.swap(0, 4);
        self.corners.swap(3, 7);

        self.edges.swap(3, 11);
        self.edges.swap(4, 7);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_orientation_wraps_modulo_2() {
        let cases = [(0, 0), (1, 1), (2, 0), (3, 1)];

        for (input, expected) in cases {
            let orientation = EdgeOrientation::new(input);

            assert_eq!(orientation.as_u8(), expected);
        }
    }

    #[test]
    fn corner_orientation_wraps_modulo_3() {
        let cases = [(0, 0), (1, 1), (2, 2), (3, 0), (4, 1), (5, 2)];

        for (input, expected) in cases {
            let orientation = CornerOrientation::new(input);

            assert_eq!(orientation.as_u8(), expected);
        }
    }

    #[test]
    fn edge_slot_accepts_valid_input() {
        for value in 0..12 {
            let slot = EdgeSlot::new(value);
            assert!(slot.is_ok(), "expected slot {value} to be valid");
            assert_eq!(slot.unwrap().as_u8(), value);
        }
    }

    #[test]
    fn edge_slot_rejects_invalid_input() {
        assert_eq!(
            EdgeSlot::new(12),
            Err(CubeError::InvalidEdgeSlot { value: 12 })
        );
    }

    #[test]
    fn corner_slot_accepts_valid_input() {
        for value in 0..8 {
            let slot = CornerSlot::new(value);
            assert!(slot.is_ok(), "expected slot {value} to be valid");
            assert_eq!(slot.unwrap().as_u8(), value)
        }
    }

    #[test]
    fn corner_slot_rejects_invalid_input() {
        assert_eq!(
            CornerSlot::new(8),
            Err(CubeError::InvalidCornerSlot { value: 8 })
        );
    }

    #[test]
    fn solved_cube_is_solved() {
        assert!(Cube::solved().is_solved());
    }

    #[test]
    fn solved_cube_has_no_edge_parity() {
        assert_eq!(Cube::solved().edge_parity(), 0);
    }

    #[test]
    fn solved_cube_has_no_corner_parity() {
        assert_eq!(Cube::solved().corner_parity(), 0);
    }

    #[test]
    fn solved_cube_has_all_different_edges() {
        assert!(Cube::solved().all_different_edges())
    }

    #[test]
    fn solved_cube_has_all_different_corners() {
        assert!(Cube::solved().all_different_corners())
    }
}
