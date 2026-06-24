use crate::rubiks_core::{CornerPiece, Cube, EdgePiece, StickerColor};

pub struct DerivedStickers {
    stickers: [[StickerColor; 9]; 6],
}

impl DerivedStickers {
    pub fn from_cube(cube: &Cube) -> Self {
        let mut stickers = [[StickerColor::White; 9]; 6];
        //White Face
        stickers[0] = [
            cube.corners()[0].get_sticker_colors()[0],
            cube.edges()[0].get_sticker_colors()[0],
            cube.corners()[1].get_sticker_colors()[0],
            cube.edges()[3].get_sticker_colors()[0],
            StickerColor::White,
            cube.edges()[1].get_sticker_colors()[0],
            cube.corners()[3].get_sticker_colors()[0],
            cube.edges()[2].get_sticker_colors()[0],
            cube.corners()[2].get_sticker_colors()[0],
        ];
        //Yellow Face
        stickers[1] = [
            cube.corners()[4].get_sticker_colors()[0],
            cube.edges()[8].get_sticker_colors()[0],
            cube.corners()[5].get_sticker_colors()[0],
            cube.edges()[11].get_sticker_colors()[0],
            StickerColor::Yellow,
            cube.edges()[9].get_sticker_colors()[0],
            cube.corners()[7].get_sticker_colors()[0],
            cube.edges()[10].get_sticker_colors()[0],
            cube.corners()[6].get_sticker_colors()[0],
        ];
        //Blue Face
        stickers[2] = [
            cube.corners()[7].get_sticker_colors()[1],
            cube.edges()[10].get_sticker_colors()[1],
            cube.corners()[6].get_sticker_colors()[2],
            cube.edges()[4].get_sticker_colors()[0],
            StickerColor::Blue,
            cube.edges()[5].get_sticker_colors()[0],
            cube.corners()[0].get_sticker_colors()[2],
            cube.edges()[0].get_sticker_colors()[1],
            cube.corners()[1].get_sticker_colors()[1],
        ];
        //Green Face
        stickers[3] = [
            cube.corners()[3].get_sticker_colors()[1],
            cube.edges()[2].get_sticker_colors()[1],
            cube.corners()[2].get_sticker_colors()[2],
            cube.edges()[7].get_sticker_colors()[0],
            StickerColor::Green,
            cube.edges()[6].get_sticker_colors()[0],
            cube.corners()[4].get_sticker_colors()[2],
            cube.edges()[8].get_sticker_colors()[1],
            cube.corners()[5].get_sticker_colors()[1],
        ];
        //Red Face
        stickers[4] = [
            cube.corners()[2].get_sticker_colors()[1],
            cube.edges()[1].get_sticker_colors()[1],
            cube.corners()[1].get_sticker_colors()[2],
            cube.edges()[6].get_sticker_colors()[1],
            StickerColor::Red,
            cube.edges()[5].get_sticker_colors()[1],
            cube.corners()[5].get_sticker_colors()[2],
            cube.edges()[9].get_sticker_colors()[1],
            cube.corners()[6].get_sticker_colors()[1],
        ];
        //Orange Face
        stickers[5] = [
            cube.corners()[0].get_sticker_colors()[1],
            cube.edges()[3].get_sticker_colors()[1],
            cube.corners()[3].get_sticker_colors()[2],
            cube.edges()[4].get_sticker_colors()[1],
            StickerColor::Orange,
            cube.edges()[7].get_sticker_colors()[1],
            cube.corners()[7].get_sticker_colors()[2],
            cube.edges()[11].get_sticker_colors()[1],
            cube.corners()[4].get_sticker_colors()[1],
        ];
        Self { stickers }
    }
    pub fn print_cube_net(&self) {
        let mut sticker_chars = [['-'; 9]; 6];
        for (face_idx, face) in self.stickers.iter().enumerate() {
            for (sticker_idx, sticker_color) in face.iter().enumerate() {
                match *sticker_color {
                    StickerColor::White => sticker_chars[face_idx][sticker_idx] = 'W',
                    StickerColor::Yellow => sticker_chars[face_idx][sticker_idx] = 'Y',
                    StickerColor::Blue => sticker_chars[face_idx][sticker_idx] = 'B',
                    StickerColor::Green => sticker_chars[face_idx][sticker_idx] = 'G',
                    StickerColor::Red => sticker_chars[face_idx][sticker_idx] = 'R',
                    StickerColor::Orange => sticker_chars[face_idx][sticker_idx] = 'O',
                }
            }
        }
        println!(
            "      {} {} {}      ",
            sticker_chars[0][0], sticker_chars[0][1], sticker_chars[0][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[0][3], sticker_chars[0][4], sticker_chars[0][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[0][6], sticker_chars[0][7], sticker_chars[0][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[5][0], sticker_chars[5][1], sticker_chars[5][2]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][0], sticker_chars[3][1], sticker_chars[3][2]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][0], sticker_chars[4][1], sticker_chars[4][2]
        );
        println!();
        print!(
            "{} {} {} ",
            sticker_chars[5][3], sticker_chars[5][4], sticker_chars[5][5]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][3], sticker_chars[3][4], sticker_chars[3][5]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][3], sticker_chars[4][4], sticker_chars[4][5]
        );
        println!();
        print!(
            "{} {} {} ",
            sticker_chars[5][6], sticker_chars[5][7], sticker_chars[5][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[3][6], sticker_chars[3][7], sticker_chars[3][8]
        );
        print!(
            "{} {} {} ",
            sticker_chars[4][6], sticker_chars[4][7], sticker_chars[4][8]
        );
        println!();
        println!(
            "      {} {} {}      ",
            sticker_chars[1][0], sticker_chars[1][1], sticker_chars[1][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[1][3], sticker_chars[1][4], sticker_chars[1][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[1][6], sticker_chars[1][7], sticker_chars[1][8]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][0], sticker_chars[2][1], sticker_chars[2][2]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][3], sticker_chars[2][4], sticker_chars[2][5]
        );
        println!(
            "      {} {} {}      ",
            sticker_chars[2][6], sticker_chars[2][7], sticker_chars[2][8]
        );
    }
}
