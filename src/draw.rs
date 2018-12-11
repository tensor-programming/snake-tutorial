use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 15.0;

const DIGIT_WIDTH: usize = 4;
const DIGIT_HEIGHT: usize = 4;

const DIGIT_FILL: [[bool; DIGIT_WIDTH * DIGIT_HEIGHT]; 10] = [
    [false,  true,  true, false,
     false,  true, false,  true,
     false,  true, false,  true,
     false, false,  true,  true,
    ], // 0
    [false,  true,  true,  true,
     false, false,  true,  true,
     false, false,  true,  true,
     false, false,  true,  true,
    ], // 1
    [false,  true,  true,  true,
     false, false, false,  true,
     false,  true,  true, false,
     false,  true,  true,  true,
    ], // 2
    [false,  true,  true,  true,
     false, false, false,  true,
     false, false,  true,  true,
     false,  true,  true,  true,
    ], // 3
    [false,  true, false,  true,
     false,  true,  true,  true,
     false,  true,  true,  true,
     false, false, false,  true,
    ], // 4
    [false,  true, true,  true,
     false,  true, false, false,
     false, false,  true,  true,
     false,  true,  true,  true,
    ], // 5
    [false, false,  true,  true,
     false,  true, false, false,
     false,  true,  true,  true,
     false,  true,  true,  true,
    ], // 6
    [false,  true,  true,  true,
     false, false, false,  true,
     false, false,  true,  true,
     false, false, false,  true,
    ], // 7
    [false,  true,  true,  true,
     false,  true, false,  true,
     false,  true,  true,  true,
     false,  true,  true,  true,
    ], // 8
    [false,  true,  true,  true,
     false,  true,  true,  true,
     false, false, false,  true,
     false, false, false,  true,
    ], // 9
];

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_number(
    color: Color,
    number: u32,
    x: i32,
    y: i32,
    con: &Context,
    g: &mut G2d,
) {
    let mut num = number;
    let mut left_start = x - DIGIT_WIDTH as i32;

    loop {
        let digit = num as usize % 10;

        for i in 0..(DIGIT_WIDTH * DIGIT_HEIGHT) {
            let fill = DIGIT_FILL[digit][i];
            let draw_x = left_start + (i % DIGIT_WIDTH) as i32;
            let draw_y = y + (i / DIGIT_WIDTH) as i32;

            if fill {
                draw_block(color, draw_x, draw_y, con, g);
            }
        }

        left_start -= DIGIT_WIDTH as i32;
        num /= 10;
        if num == 0 {
            break;
        }
    }
}
