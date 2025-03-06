// NOTE: These spraycan definitions are from info.c, starting from line
// 22080 (on commit 03241a13c5b22f567577c2cb06c4bbfb2f8e3cc9).
//
// There is no script to generate these atm; I just use a multicursor editor
// and some tricks to get it the way I want, and `rust-fmt` takes care of the rest.

use super::Spray;

pub fn sprays() -> Vec<Spray> {
    vec![
        Spray {
            name: "Default".into(),
            ramp: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            id: "SKINCOLOR_NONE".into(),
        },
        Spray {
            name: "White".into(),
            ramp: [0, 0, 0, 0, 1, 2, 5, 8, 9, 11, 14, 17, 20, 22, 25, 28],
            id: "SKINCOLOR_WHITE".into(),
        },
        Spray {
            name: "Silver".into(),
            ramp: [0, 1, 2, 3, 5, 7, 9, 12, 13, 15, 18, 20, 23, 25, 27, 30],
            id: "SKINCOLOR_SILVER".into(),
        },
        Spray {
            name: "Grey".into(),
            ramp: [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31],
            id: "SKINCOLOR_GREY".into(),
        },
        Spray {
            name: "Nickel".into(),
            ramp: [3, 5, 8, 11, 15, 17, 19, 21, 23, 24, 25, 26, 27, 29, 30, 31],
            id: "SKINCOLOR_NICKEL".into(),
        },
        Spray {
            name: "Black".into(),
            ramp: [4, 7, 11, 15, 20, 22, 24, 27, 28, 28, 28, 29, 29, 30, 30, 31],
            id: "SKINCOLOR_BLACK".into(),
        },
        Spray {
            name: "Skunk".into(),
            ramp: [0, 1, 2, 3, 4, 10, 16, 21, 23, 24, 25, 26, 27, 28, 29, 31],
            id: "SKINCOLOR_SKUNK".into(),
        },
        Spray {
            name: "Fairy".into(),
            ramp: [
                0, 0, 252, 252, 200, 201, 211, 14, 16, 18, 20, 22, 24, 26, 28, 31,
            ],
            id: "SKINCOLOR_FAIRY".into(),
        },
        Spray {
            name: "Popcorn".into(),
            ramp: [
                0, 80, 80, 81, 82, 218, 240, 11, 13, 16, 18, 21, 23, 26, 28, 31,
            ],
            id: "SKINCOLOR_POPCORN".into(),
        },
        Spray {
            name: "Artichoke".into(),
            ramp: [
                80, 88, 89, 98, 99, 91, 12, 14, 16, 18, 20, 22, 24, 26, 28, 31,
            ],
            id: "SKINCOLOR_ARTICHOKE".into(),
        },
        Spray {
            name: "Pigeon".into(),
            ramp: [
                0, 128, 129, 130, 146, 170, 14, 15, 17, 19, 21, 23, 25, 27, 29, 31,
            ],
            id: "SKINCOLOR_PIGEON".into(),
        },
        Spray {
            name: "Sepia".into(),
            ramp: [
                0, 1, 3, 5, 7, 9, 241, 242, 243, 245, 247, 249, 236, 237, 238, 239,
            ],
            id: "SKINCOLOR_SEPIA".into(),
        },
        Spray {
            name: "Beige".into(),
            ramp: [
                0, 208, 216, 217, 240, 241, 242, 243, 245, 247, 249, 250, 251, 237, 238, 239,
            ],
            id: "SKINCOLOR_BEIGE".into(),
        },
        Spray {
            name: "Caramel".into(),
            ramp: [
                208, 48, 216, 217, 218, 220, 221, 223, 224, 226, 228, 230, 232, 234, 236, 239,
            ],
            id: "SKINCOLOR_CARAMEL".into(),
        },
        Spray {
            name: "Peach".into(),
            ramp: [
                0, 208, 48, 216, 218, 221, 212, 213, 214, 215, 206, 207, 197, 198, 199, 254,
            ],
            id: "SKINCOLOR_PEACH".into(),
        },
        Spray {
            name: "Brown".into(),
            ramp: [
                216, 217, 219, 221, 224, 225, 227, 229, 230, 232, 234, 235, 237, 239, 29, 30,
            ],
            id: "SKINCOLOR_BROWN".into(),
        },
        Spray {
            name: "Leather".into(),
            ramp: [
                218, 221, 224, 227, 229, 231, 233, 235, 237, 239, 28, 28, 29, 29, 30, 31,
            ],
            id: "SKINCOLOR_LEATHER".into(),
        },
        Spray {
            name: "Pink".into(),
            ramp: [
                0, 208, 208, 209, 209, 210, 211, 211, 212, 213, 214, 215, 41, 43, 45, 46,
            ],
            id: "SKINCOLOR_PINK".into(),
        },
        Spray {
            name: "Rose".into(),
            ramp: [
                209, 210, 211, 211, 212, 213, 214, 215, 41, 42, 43, 44, 45, 71, 46, 47,
            ],
            id: "SKINCOLOR_ROSE".into(),
        },
        Spray {
            name: "Cinnamon".into(),
            ramp: [
                216, 221, 224, 226, 228, 60, 61, 43, 44, 45, 71, 46, 47, 29, 30, 31,
            ],
            id: "SKINCOLOR_CINNAMON".into(),
        },
        Spray {
            name: "Ruby".into(),
            ramp: [
                0, 208, 209, 210, 211, 213, 39, 40, 41, 43, 186, 186, 169, 169, 253, 254,
            ],
            id: "SKINCOLOR_RUBY".into(),
        },
        Spray {
            name: "Raspberry".into(),
            ramp: [
                0, 208, 209, 210, 32, 33, 34, 35, 37, 39, 41, 43, 44, 45, 46, 47,
            ],
            id: "SKINCOLOR_RASPBERRY".into(),
        },
        Spray {
            name: "Red".into(),
            ramp: [
                209, 210, 32, 34, 36, 38, 39, 40, 41, 42, 43, 44, 45, 71, 46, 47,
            ],
            id: "SKINCOLOR_RED".into(),
        },
        Spray {
            name: "Crimson".into(),
            ramp: [
                210, 33, 35, 38, 40, 42, 43, 45, 71, 71, 46, 46, 47, 47, 30, 31,
            ],
            id: "SKINCOLOR_CRIMSON".into(),
        },
        Spray {
            name: "Maroon".into(),
            ramp: [
                32, 33, 35, 37, 39, 41, 43, 237, 26, 26, 27, 27, 28, 29, 30, 31,
            ],
            id: "SKINCOLOR_MAROON".into(),
        },
        Spray {
            name: "Lemonade".into(),
            ramp: [
                0, 80, 81, 82, 83, 216, 210, 211, 212, 213, 214, 215, 43, 44, 71, 47,
            ],
            id: "SKINCOLOR_LEMONADE".into(),
        },
        Spray {
            name: "Scarlet".into(),
            ramp: [
                48, 49, 50, 51, 53, 34, 36, 38, 184, 185, 168, 168, 169, 169, 254, 31,
            ],
            id: "SKINCOLOR_SCARLET".into(),
        },
        Spray {
            name: "Ketchup".into(),
            ramp: [
                72, 73, 64, 51, 52, 54, 34, 36, 38, 40, 42, 43, 44, 71, 46, 47,
            ],
            id: "SKINCOLOR_KETCHUP".into(),
        },
        Spray {
            name: "Dawn".into(),
            ramp: [
                0, 208, 216, 209, 210, 211, 212, 57, 58, 59, 60, 61, 63, 71, 47, 31,
            ],
            id: "SKINCOLOR_DAWN".into(),
        },
        Spray {
            name: "Sunslam".into(),
            ramp: [
                82, 72, 73, 64, 51, 53, 55, 213, 214, 195, 195, 173, 174, 175, 253, 254,
            ],
            id: "SKINCOLOR_SUNSLAM".into(),
        },
        Spray {
            name: "Creamsicle".into(),
            ramp: [
                0, 0, 208, 208, 48, 49, 50, 52, 53, 54, 56, 57, 58, 60, 61, 63,
            ],
            id: "SKINCOLOR_CREAMSICLE".into(),
        },
        Spray {
            name: "Orange".into(),
            ramp: [
                208, 48, 49, 50, 51, 52, 53, 54, 55, 57, 59, 60, 62, 44, 71, 47,
            ],
            id: "SKINCOLOR_ORANGE".into(),
        },
        Spray {
            name: "Rosewood".into(),
            ramp: [
                50, 52, 55, 56, 58, 59, 60, 61, 62, 63, 44, 45, 71, 46, 47, 30,
            ],
            id: "SKINCOLOR_ROSEWOOD".into(),
        },
        Spray {
            name: "Tangerine".into(),
            ramp: [
                80, 81, 82, 83, 64, 51, 52, 54, 55, 57, 58, 60, 61, 63, 71, 47,
            ],
            id: "SKINCOLOR_TANGERINE".into(),
        },
        Spray {
            name: "Tan".into(),
            ramp: [
                0, 80, 81, 82, 83, 84, 85, 86, 87, 245, 246, 248, 249, 251, 237, 239,
            ],
            id: "SKINCOLOR_TAN".into(),
        },
        Spray {
            name: "Cream".into(),
            ramp: [
                0, 80, 80, 81, 81, 49, 51, 222, 224, 227, 230, 233, 236, 239, 29, 31,
            ],
            id: "SKINCOLOR_CREAM".into(),
        },
        Spray {
            name: "Gold".into(),
            ramp: [
                0, 80, 81, 83, 64, 65, 66, 67, 68, 215, 69, 70, 44, 71, 46, 47,
            ],
            id: "SKINCOLOR_GOLD".into(),
        },
        Spray {
            name: "Royal".into(),
            ramp: [
                80, 81, 83, 64, 65, 223, 229, 196, 196, 197, 197, 198, 199, 29, 30, 31,
            ],
            id: "SKINCOLOR_ROYAL".into(),
        },
        Spray {
            name: "Bronze".into(),
            ramp: [
                83, 64, 65, 66, 67, 215, 69, 70, 44, 44, 45, 71, 46, 47, 29, 31,
            ],
            id: "SKINCOLOR_BRONZE".into(),
        },
        Spray {
            name: "Copper".into(),
            ramp: [
                0, 82, 64, 65, 67, 68, 70, 237, 239, 28, 28, 29, 29, 30, 30, 31,
            ],
            id: "SKINCOLOR_COPPER".into(),
        },
        Spray {
            name: "Yellow".into(),
            ramp: [
                0, 80, 81, 82, 83, 73, 84, 74, 64, 65, 66, 67, 68, 69, 70, 71,
            ],
            id: "SKINCOLOR_YELLOW".into(),
        },
        Spray {
            name: "Mustard".into(),
            ramp: [
                80, 81, 82, 83, 64, 65, 65, 76, 76, 77, 77, 78, 79, 237, 239, 29,
            ],
            id: "SKINCOLOR_MUSTARD".into(),
        },
        Spray {
            name: "Banana".into(),
            ramp: [
                80, 81, 83, 72, 73, 74, 75, 76, 77, 78, 79, 236, 237, 238, 239, 30,
            ],
            id: "SKINCOLOR_BANANA".into(),
        },
        Spray {
            name: "Olive".into(),
            ramp: [
                80, 82, 73, 74, 75, 76, 77, 78, 79, 236, 237, 238, 239, 28, 29, 31,
            ],
            id: "SKINCOLOR_OLIVE".into(),
        },
        Spray {
            name: "Crocodile".into(),
            ramp: [
                0, 80, 81, 88, 88, 188, 189, 76, 76, 77, 78, 79, 236, 237, 238, 239,
            ],
            id: "SKINCOLOR_CROCODILE".into(),
        },
        Spray {
            name: "Peridot".into(),
            ramp: [
                0, 80, 81, 88, 188, 189, 190, 191, 94, 94, 95, 95, 109, 110, 111, 31,
            ],
            id: "SKINCOLOR_PERIDOT".into(),
        },
        Spray {
            name: "Vomit".into(),
            ramp: [
                0, 208, 216, 209, 218, 51, 65, 76, 191, 191, 126, 143, 138, 175, 169, 254,
            ],
            id: "SKINCOLOR_VOMIT".into(),
        },
        Spray {
            name: "Garden".into(),
            ramp: [
                81, 82, 83, 73, 64, 65, 66, 92, 92, 93, 93, 94, 95, 109, 110, 111,
            ],
            id: "SKINCOLOR_GARDEN".into(),
        },
        Spray {
            name: "Lime".into(),
            ramp: [
                0, 80, 81, 88, 188, 189, 114, 114, 115, 115, 116, 116, 117, 118, 119, 111,
            ],
            id: "SKINCOLOR_LIME".into(),
        },
        Spray {
            name: "Handheld".into(),
            ramp: [
                83, 72, 73, 74, 75, 76, 102, 104, 105, 106, 107, 108, 109, 110, 111, 31,
            ],
            id: "SKINCOLOR_HANDHELD".into(),
        },
        Spray {
            name: "Tea".into(),
            ramp: [
                0, 80, 80, 81, 88, 89, 90, 91, 92, 93, 94, 95, 109, 110, 111, 31,
            ],
            id: "SKINCOLOR_TEA".into(),
        },
        Spray {
            name: "Pistachio".into(),
            ramp: [
                0, 80, 88, 88, 89, 90, 91, 102, 103, 104, 105, 106, 107, 108, 109, 110,
            ],
            id: "SKINCOLOR_PISTACHIO".into(),
        },
        Spray {
            name: "Moss".into(),
            ramp: [
                88, 89, 90, 91, 91, 92, 93, 94, 107, 107, 108, 108, 109, 109, 110, 111,
            ],
            id: "SKINCOLOR_MOSS".into(),
        },
        Spray {
            name: "Camouflage".into(),
            ramp: [
                208, 84, 85, 240, 241, 243, 245, 94, 107, 108, 108, 109, 109, 110, 110, 111,
            ],
            id: "SKINCOLOR_CAMOUFLAGE".into(),
        },
        Spray {
            name: "Mint".into(),
            ramp: [
                0, 88, 88, 89, 89, 100, 101, 102, 125, 126, 143, 143, 138, 175, 169, 254,
            ],
            id: "SKINCOLOR_MINT".into(),
        },
        Spray {
            name: "Green".into(),
            ramp: [
                96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
            ],
            id: "SKINCOLOR_GREEN".into(),
        },
        Spray {
            name: "Pinetree".into(),
            ramp: [
                97, 99, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 30, 30, 31,
            ],
            id: "SKINCOLOR_PINETREE".into(),
        },
        Spray {
            name: "Turtle".into(),
            ramp: [
                96, 112, 112, 113, 113, 114, 114, 115, 115, 116, 116, 117, 117, 118, 119, 111,
            ],
            id: "SKINCOLOR_TURTLE".into(),
        },
        Spray {
            name: "Swamp".into(),
            ramp: [
                96, 112, 113, 114, 115, 116, 117, 118, 119, 119, 29, 29, 30, 30, 31, 31,
            ],
            id: "SKINCOLOR_SWAMP".into(),
        },
        Spray {
            name: "Dream".into(),
            ramp: [
                0, 0, 208, 208, 48, 89, 98, 100, 148, 148, 172, 172, 173, 173, 174, 175,
            ],
            id: "SKINCOLOR_DREAM".into(),
        },
        Spray {
            name: "Plague".into(),
            ramp: [
                80, 88, 96, 112, 113, 124, 142, 149, 149, 173, 174, 175, 169, 253, 254, 31,
            ],
            id: "SKINCOLOR_PLAGUE".into(),
        },
        Spray {
            name: "Emerald".into(),
            ramp: [
                0, 120, 121, 112, 113, 114, 115, 125, 125, 126, 126, 127, 138, 175, 253, 254,
            ],
            id: "SKINCOLOR_EMERALD".into(),
        },
        Spray {
            name: "Algae".into(),
            ramp: [
                128, 129, 130, 131, 132, 133, 134, 115, 115, 116, 116, 117, 118, 119, 110, 111,
            ],
            id: "SKINCOLOR_ALGAE".into(),
        },
        Spray {
            name: "Aquamarine".into(),
            ramp: [
                0, 128, 120, 121, 122, 123, 124, 125, 126, 126, 127, 127, 118, 118, 119, 111,
            ],
            id: "SKINCOLOR_AQUAMARINE".into(),
        },
        Spray {
            name: "Turquoise".into(),
            ramp: [
                128, 120, 121, 122, 123, 141, 141, 142, 142, 143, 143, 138, 138, 139, 139, 31,
            ],
            id: "SKINCOLOR_TURQUOISE".into(),
        },
        Spray {
            name: "Teal".into(),
            ramp: [
                0, 120, 120, 121, 140, 141, 142, 143, 143, 138, 138, 139, 139, 254, 254, 31,
            ],
            id: "SKINCOLOR_TEAL".into(),
        },
        Spray {
            name: "Robin".into(),
            ramp: [
                0, 80, 81, 82, 83, 88, 121, 140, 133, 133, 134, 135, 136, 137, 138, 139,
            ],
            id: "SKINCOLOR_ROBIN".into(),
        },
        Spray {
            name: "Cyan".into(),
            ramp: [
                0, 0, 128, 128, 255, 131, 132, 134, 142, 142, 143, 127, 118, 119, 110, 111,
            ],
            id: "SKINCOLOR_CYAN".into(),
        },
        Spray {
            name: "Jawz".into(),
            ramp: [
                0, 0, 128, 128, 129, 146, 133, 134, 135, 149, 149, 173, 173, 174, 175, 31,
            ],
            id: "SKINCOLOR_JAWZ".into(),
        },
        Spray {
            name: "Cerulean".into(),
            ramp: [
                0, 128, 129, 130, 131, 132, 133, 135, 136, 136, 137, 137, 138, 138, 139, 31,
            ],
            id: "SKINCOLOR_CERULEAN".into(),
        },
        Spray {
            name: "Navy".into(),
            ramp: [
                128, 129, 130, 132, 134, 135, 136, 137, 137, 138, 138, 139, 139, 29, 30, 31,
            ],
            id: "SKINCOLOR_NAVY".into(),
        },
        Spray {
            name: "Platinum".into(),
            ramp: [
                0, 0, 0, 144, 144, 145, 9, 11, 14, 142, 136, 137, 138, 138, 139, 31,
            ],
            id: "SKINCOLOR_PLATINUM".into(),
        },
        Spray {
            name: "Slate".into(),
            ramp: [
                0, 0, 144, 144, 144, 145, 145, 145, 170, 170, 171, 171, 172, 173, 174, 175,
            ],
            id: "SKINCOLOR_SLATE".into(),
        },
        Spray {
            name: "Steel".into(),
            ramp: [
                0, 144, 144, 145, 145, 170, 170, 171, 171, 172, 172, 173, 173, 174, 175, 31,
            ],
            id: "SKINCOLOR_STEEL".into(),
        },
        Spray {
            name: "Thunder".into(),
            ramp: [
                80, 81, 82, 83, 64, 65, 11, 171, 172, 173, 173, 157, 158, 159, 254, 31,
            ],
            id: "SKINCOLOR_THUNDER".into(),
        },
        Spray {
            name: "Nova".into(),
            ramp: [
                0, 83, 49, 50, 51, 32, 192, 148, 148, 172, 173, 174, 175, 29, 30, 31,
            ],
            id: "SKINCOLOR_NOVA".into(),
        },
        Spray {
            name: "Rust".into(),
            ramp: [
                208, 48, 216, 217, 240, 241, 242, 171, 172, 173, 24, 25, 26, 28, 29, 31,
            ],
            id: "SKINCOLOR_RUST".into(),
        },
        Spray {
            name: "Wristwatch".into(),
            ramp: [
                48, 218, 221, 224, 227, 231, 196, 173, 173, 174, 159, 159, 253, 253, 254, 31,
            ],
            id: "SKINCOLOR_WRISTWATCH".into(),
        },
        Spray {
            name: "Jet".into(),
            ramp: [
                145, 146, 147, 148, 149, 173, 173, 174, 175, 175, 28, 28, 29, 29, 30, 31,
            ],
            id: "SKINCOLOR_JET".into(),
        },
        Spray {
            name: "Sapphire".into(),
            ramp: [
                0, 128, 129, 131, 133, 135, 149, 150, 152, 154, 156, 158, 159, 253, 254, 31,
            ],
            id: "SKINCOLOR_SAPPHIRE".into(),
        },
        Spray {
            name: "Ultramarine".into(),
            ramp: [
                0, 0, 120, 120, 121, 133, 135, 149, 149, 166, 166, 167, 168, 169, 254, 31,
            ],
            id: "SKINCOLOR_ULTRAMARINE".into(),
        },
        Spray {
            name: "Periwinkle".into(),
            ramp: [
                0, 0, 144, 144, 145, 146, 147, 149, 150, 152, 154, 155, 157, 159, 253, 254,
            ],
            id: "SKINCOLOR_PERIWINKLE".into(),
        },
        Spray {
            name: "Blue".into(),
            ramp: [
                144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 155, 156, 158, 253, 254, 31,
            ],
            id: "SKINCOLOR_BLUE".into(),
        },
        Spray {
            name: "Midnight".into(),
            ramp: [
                146, 148, 149, 150, 152, 153, 155, 157, 159, 253, 253, 254, 254, 31, 31, 31,
            ],
            id: "SKINCOLOR_MIDNIGHT".into(),
        },
        Spray {
            name: "Blueberry".into(),
            ramp: [
                0, 144, 145, 146, 147, 171, 172, 166, 166, 167, 167, 168, 168, 175, 169, 253,
            ],
            id: "SKINCOLOR_BLUEBERRY".into(),
        },
        Spray {
            name: "Thistle".into(),
            ramp: [
                0, 0, 0, 252, 252, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 254,
            ],
            id: "SKINCOLOR_THISTLE".into(),
        },
        Spray {
            name: "Purple".into(),
            ramp: [
                0, 252, 160, 161, 162, 163, 164, 165, 166, 167, 168, 168, 169, 169, 253, 254,
            ],
            id: "SKINCOLOR_PURPLE".into(),
        },
        Spray {
            name: "Pastel".into(),
            ramp: [
                0, 128, 128, 129, 129, 146, 170, 162, 163, 164, 165, 166, 167, 168, 169, 254,
            ],
            id: "SKINCOLOR_PASTEL".into(),
        },
        Spray {
            name: "Moonset".into(),
            ramp: [
                0, 144, 145, 146, 170, 162, 163, 184, 184, 207, 207, 44, 45, 46, 47, 31,
            ],
            id: "SKINCOLOR_MOONSET".into(),
        },
        Spray {
            name: "Dusk".into(),
            ramp: [
                252, 200, 201, 192, 193, 194, 172, 172, 173, 173, 174, 174, 175, 169, 253, 254,
            ],
            id: "SKINCOLOR_DUSK".into(),
        },
        Spray {
            name: "Violet".into(),
            ramp: [
                176, 177, 178, 179, 180, 181, 182, 183, 184, 165, 165, 166, 167, 168, 169, 254,
            ],
            id: "SKINCOLOR_VIOLET".into(),
        },
        Spray {
            name: "Magenta".into(),
            ramp: [
                252, 200, 177, 177, 178, 179, 180, 181, 182, 183, 183, 184, 185, 186, 187, 31,
            ],
            id: "SKINCOLOR_MAGENTA".into(),
        },
        Spray {
            name: "Fuchsia".into(),
            ramp: [
                208, 209, 209, 32, 33, 182, 183, 184, 185, 185, 186, 186, 187, 253, 254, 31,
            ],
            id: "SKINCOLOR_FUCHSIA".into(),
        },
        Spray {
            name: "Toxic".into(),
            ramp: [
                0, 0, 88, 88, 89, 6, 8, 10, 193, 194, 195, 184, 185, 186, 187, 31,
            ],
            id: "SKINCOLOR_TOXIC".into(),
        },
        Spray {
            name: "Mauve".into(),
            ramp: [
                80, 81, 82, 83, 64, 50, 201, 192, 193, 194, 195, 173, 174, 175, 253, 254,
            ],
            id: "SKINCOLOR_MAUVE".into(),
        },
        Spray {
            name: "Lavender".into(),
            ramp: [
                252, 177, 179, 192, 193, 194, 195, 196, 196, 197, 197, 198, 198, 199, 30, 31,
            ],
            id: "SKINCOLOR_LAVENDER".into(),
        },
        Spray {
            name: "Byzantium".into(),
            ramp: [
                145, 192, 193, 194, 195, 196, 197, 198, 199, 199, 29, 29, 30, 30, 31, 31,
            ],
            id: "SKINCOLOR_BYZANTIUM".into(),
        },
        Spray {
            name: "Pomegranate".into(),
            ramp: [
                208, 209, 210, 211, 212, 213, 214, 195, 195, 196, 196, 197, 198, 199, 29, 30,
            ],
            id: "SKINCOLOR_POMEGRANATE".into(),
        },
        Spray {
            name: "Lilac".into(),
            ramp: [
                0, 0, 0, 252, 252, 176, 200, 201, 179, 192, 193, 194, 195, 196, 197, 198,
            ],
            id: "SKINCOLOR_LILAC".into(),
        },
        Spray {
            name: "Blossom".into(),
            ramp: [
                0, 252, 252, 176, 200, 177, 201, 202, 202, 34, 36, 38, 40, 42, 45, 46,
            ],
            id: "SKINCOLOR_BLOSSOM".into(),
        },
        Spray {
            name: "Taffy".into(),
            ramp: [
                0, 252, 252, 200, 200, 201, 202, 203, 204, 204, 205, 206, 207, 43, 45, 47,
            ],
            id: "SKINCOLOR_TAFFY".into(),
        },
    ]
}
