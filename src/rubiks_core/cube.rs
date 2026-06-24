#[derive(Debug, PartialEq)]
enum CubeError {
    InvalidEdgeSlot { value: u8 },
    InvalidCornerSlot { value: u8 },
}

enum Sticker {
    White,
    Yellow,
    Blue,
    Green,
    Orange,
    Red,
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
    orientation: EdgeOrientation,
    slot: EdgeSlot,
    stickers: (Sticker, Sticker),
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
    orientation: CornerOrientation,
    slot: CornerSlot,
    stickers: (Sticker, Sticker, Sticker),
}

struct Cube {
    corners: [CornerPiece; 8],
    edges: [EdgePiece; 12],
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
