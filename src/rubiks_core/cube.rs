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

#[derive(Debug, PartialEq)]
enum CubeError {
    InvalidEdgeSlot { value: u8 },
    InvalidCornerSlot { value: u8 },
}

enum EdgeCubie {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EdgeOrientation(u8);

impl EdgeOrientation {
    fn new(value: u8) -> Self {
        Self(value % 2)
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EdgeSlot(u8);

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

struct EdgePiece {
    cubie: EdgeCubie,
    orientation: EdgeOrientation,
}

enum CornerCubie {
    WOB,
    WBR,
    WRG,
    WGO,
    YRB,
    YBO,
    YOG,
    YGR,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CornerOrientation(u8);

impl CornerOrientation {
    fn new(value: u8) -> Self {
        CornerOrientation(value % 3)
    }
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CornerSlot(u8);

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

struct CornerPiece {
    cubie: CornerCubie,
    orientation: CornerOrientation,
}

struct Cube {
    corners: [CornerPiece; 8],
    edges: [EdgePiece; 12],
}

impl Cube {
    fn new(corners: [CornerPiece; 8], edges: [EdgePiece; 12]) -> Self {
        Self { corners, edges }
    }
    fn solved() -> Self {
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
}
