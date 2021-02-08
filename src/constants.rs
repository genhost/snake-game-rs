use crate::snake::SnakeDir;

pub const DEFAULT_SNAKE_DIR: SnakeDir = SnakeDir::Right;
pub const DEFAULT_SNAKE_COLOR: i16 = COLOR_PAIR_GREEN;
pub const DEFAULT_SNAKE_SIZE: usize = 5;
pub const DEFAULT_PART_CHAR: char = '#';
pub const DEFAULT_APPLE_CHAR: char = '#';
pub const DEFAULT_APPLE_COLOR: i16 = COLOR_PAIR_RED;
pub const DEFAULT_APPLE_INCSIZE: usize = 5;
pub const DEFAULT_DISPLAY_STATE: &str = "points";

pub const COLOR_PAIR_WHITE: i16 = 1;
pub const COLOR_PAIR_BLUE: i16 = 2;
pub const COLOR_PAIR_CYAN: i16 = 3;
pub const COLOR_PAIR_GREEN: i16 = 4;
pub const COLOR_PAIR_RED: i16 = 5;
pub const COLOR_PAIR_YELLOW: i16 = 6;
pub const COLOR_PAIR_MAGENTA: i16 = 7;
pub const COLOR_PAIR_BLACK: i16 = 8;
pub const COLOR_RANDOM: i16 = 9;

pub const COORD_CENTER: i32 = -1;
