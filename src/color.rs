use crate::constants::{
    COLOR_PAIR_BLACK, COLOR_PAIR_BLUE, COLOR_PAIR_CYAN, COLOR_PAIR_GREEN, COLOR_PAIR_MAGENTA,
    COLOR_PAIR_RED, COLOR_PAIR_WHITE, COLOR_PAIR_YELLOW,
};
use pancurses::{
    init_pair, start_color, COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA,
    COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use rand::{thread_rng, Rng};

pub fn init() {
    start_color();

    init_pair(COLOR_PAIR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(COLOR_PAIR_CYAN, COLOR_CYAN, COLOR_BLACK);
    init_pair(COLOR_PAIR_RED, COLOR_RED, COLOR_BLACK);
    init_pair(COLOR_PAIR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLUE, COLOR_BLUE, COLOR_BLACK);
    init_pair(COLOR_PAIR_GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLACK, COLOR_BLACK, COLOR_BLACK);
}

pub fn rand_color() -> u32 {
    thread_rng().gen_range(1..=7)
}
