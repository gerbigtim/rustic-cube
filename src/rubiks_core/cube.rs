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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CubeError {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeCubie {
    WB,
    WR,
    WG,
    WO,
    BO,
    BR,
    GR,
    GO,
    YB,
    YO,
    YG,
    YR,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeOrientation(u8);

impl EdgeOrientation {
    fn new(value: u8) -> Self {
        Self(value % 2)
    }
    pub fn as_u8(self) -> u8 {
        self.0
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgePiece {
    cubie: EdgeCubie,
    orientation: EdgeOrientation,
}

impl EdgePiece {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CornerCubie {
    WOB,
    WBR,
    WRG,
    WGO,
    YRB,
    YBO,
    YOG,
    YGR,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CornerOrientation(u8);

impl CornerOrientation {
    fn new(value: u8) -> Self {
        CornerOrientation(value % 3)
    }
    pub fn as_u8(self) -> u8 {
        self.0
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CornerPiece {
    cubie: CornerCubie,
    orientation: CornerOrientation,
}

impl CornerPiece {
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    corners: [CornerPiece; 8],
    edges: [EdgePiece; 12],
}

impl Cube {
    fn new(corners: [CornerPiece; 8], edges: [EdgePiece; 12]) -> Result<Self, CubeError> {
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
                    cubie: EdgeCubie::YB,
                    orientation: EdgeOrientation::new(0),
                },
                EdgePiece {
                    cubie: EdgeCubie::YO,
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
                    cubie: CornerCubie::YRB,
                    orientation: CornerOrientation::new(0),
                },
                CornerPiece {
                    cubie: CornerCubie::YBO,
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
            ],
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
