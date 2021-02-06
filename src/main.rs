#![allow(dead_code)]

use std::thread;
use pancurses::*;
use std::time::Duration;
use rand::{thread_rng, Rng};

static DEFAULT_PART_CHAR: char = '#';
static DEFAULT_SNAKE_SIZE: usize = 10;

fn main() {
    let window = initscr();
    let mut snake = Snake::new(&window, DEFAULT_PART_CHAR, DEFAULT_SNAKE_SIZE);

    start_color();

    init_pair(1, COLOR_YELLOW , COLOR_BLACK);
    init_pair(2, COLOR_CYAN   , COLOR_BLACK);
    init_pair(3, COLOR_RED    , COLOR_BLACK);
    init_pair(4, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(5, COLOR_WHITE  , COLOR_BLACK);
    init_pair(6, COLOR_BLUE   , COLOR_BLACK);
    init_pair(7, COLOR_GREEN  , COLOR_BLACK);
    // init_pair(8, COLOR_BLACK  , COLOR_BLACK);

    curs_set(0);
    noecho();

    snake.display();
    endwin();
}

struct Snake<'a> {
    parts: Vec<SnakePart>,
    window: &'a Window,
    form: char,
}

impl<'a> Snake<'a> {
    fn new(window: &'a Window, form: char, size: usize) -> Self {
        let mut parts: Vec<SnakePart> = Vec::new();
        let window_center = (window.get_max_y() / 2, window.get_max_x() / 2);
        for x_offset in 0..size {
            parts.push(SnakePart::new(window_center.0, window_center.1 + x_offset as i32))
        }
        Self {
            parts,
            window,
            form,
        }
    }

    fn display(&mut self) {
        while !self.check_game_over() {
            for part in &mut self.parts {
                let color = rand_color();

                part.mv(part.y, part.x + 1);

                self.window.attron(COLOR_PAIR(color));
                self.window.mvaddch(part.y, part.x, self.form);
                self.window.attroff(COLOR_PAIR(color));

                self.window.refresh();
            }
            thread::sleep(Duration::from_millis(100));
            self.window.clear();
        }
    }

    fn check_game_over(&self) -> bool {
        false
    }

    fn len(&self) -> usize {
        self.parts.len()
    }

    fn last_part(&self) -> &SnakePart {
        &self.parts[self.len() + 1]
    }
}

struct SnakePart {
    x: i32,
    y: i32,
}

impl SnakePart {
    fn new(y: i32, x: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn mv(&mut self, y: i32, x: i32) {
        self.y = y;
        self.x = x;
    }
}

fn rand_color() -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=7)
}
