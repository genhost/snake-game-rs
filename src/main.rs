#![allow(dead_code)]

use pancurses::*;
use rand::{thread_rng, Rng};

static DEFAULT_PART_CHAR: char = '#';
static DEFAULT_SNAKE_SIZE: usize = 10;
static DEFAULT_SNAKE_DIR: SnakeDir = SnakeDir::Right;

fn main() {
    let window = initscr();
    let mut snake = Snake::new(
        &window,
        DEFAULT_PART_CHAR,
        DEFAULT_SNAKE_SIZE,
        DEFAULT_SNAKE_DIR,
    );

    start_color();

    init_pair(1, COLOR_YELLOW, COLOR_BLACK);
    init_pair(2, COLOR_CYAN, COLOR_BLACK);
    init_pair(3, COLOR_RED, COLOR_BLACK);
    init_pair(4, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(5, COLOR_WHITE, COLOR_BLACK);
    init_pair(6, COLOR_BLUE, COLOR_BLACK);
    init_pair(7, COLOR_GREEN, COLOR_BLACK);

    curs_set(0);
    noecho();

    snake.start();

    endwin();
}

struct Snake<'a> {
    form: char,
    dir: SnakeDir,
    window: &'a Window,
    parts: Vec<SnakePart>,
    length: usize,
}

impl<'a> Snake<'a> {
    fn new(window: &'a Window, form: char, size: usize, dir: SnakeDir) -> Self {
        let mut parts: Vec<SnakePart> = Vec::new();
        let window_center = (window.get_max_y() / 2, window.get_max_x() / 2);
        for x_offset in 0..size {
            parts.push(SnakePart::new(
                window_center.0,
                window_center.1 + x_offset as i32,
            ))
        }
        Self {
            dir,
            form,
            parts,
            window,
            length: size,
        }
    }

    fn sync(&mut self) {
        let mut parts = self.parts.clone();
        parts.reverse();
        self.parts.clear();

        for i in 0..self.length {
            self.parts.push(SnakePart::new(
                parts[i].y,
                parts[i].x,
            ))
        }
        self.parts.reverse();
    }

    fn start(&mut self) {
        while !self.check_game_over() {
            self.sync();
            half_delay(1);
            self.dir = match self.window.getch() {
                Some(Input::Character('w')) => SnakeDir::Up,
                Some(Input::Character('s')) => SnakeDir::Down,
                Some(Input::Character('a')) => SnakeDir::Left,
                Some(Input::Character('d')) => SnakeDir::Right,
                Some(Input::KeyUp) => SnakeDir::Up,
                Some(Input::KeyDown) => SnakeDir::Down,
                Some(Input::KeyLeft) => SnakeDir::Left,
                Some(Input::KeyRight) => SnakeDir::Right,
                _ => self.dir,
            };

            self.mv();

            for part in &self.parts {
                let color = snake_color();

                self.window.attron(COLOR_PAIR(color));
                self.window.mvaddch(part.y, part.x, self.form);
                self.window.attroff(COLOR_PAIR(color));

                self.window.refresh();
            }
        }
    }

    fn mv(&mut self) {
        let mut last_part = self.last_part();
        match self.dir {
            SnakeDir::Up => last_part.mv(last_part.y - 1, last_part.x),
            SnakeDir::Down => last_part.mv(last_part.y + 1, last_part.x),
            SnakeDir::Left => last_part.mv(last_part.y, last_part.x - 1),
            SnakeDir::Right => last_part.mv(last_part.y, last_part.x + 1),
        }
        for part in &mut self.parts_without_last() {
            match part.dir {
                SnakeDir::Up => part.mv(part.y - 1, part.x),
                SnakeDir::Down => part.mv(part.y + 1, part.x),
                SnakeDir::Left => part.mv(part.y, part.x - 1),
                SnakeDir::Right => part.mv(part.y, part.x + 1),
            }
        }
        self.window.clear();
        self.parts.push(last_part);
    }

    fn check_game_over(&self) -> bool {
        false
    }

    fn last_part(&self) -> SnakePart {
        self.parts[self.parts.len() - 1]
    }

    fn parts_without_last(&self) -> Vec<SnakePart> {
        let mut parts: Vec<SnakePart> = Vec::new();
        for part in &self.parts {
            parts.push(*part);
        }
        parts
    }
}

#[derive(Clone, Copy)]
enum SnakeDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct SnakePart {
    x: i32,
    y: i32,
    dir: SnakeDir,
}

impl SnakePart {
    fn new(y: i32, x: i32) -> Self {
        Self {
            x,
            y,
            dir: SnakeDir::Right,
        }
    }

    fn mv(&mut self, y: i32, x: i32) {
        self.y = y;
        self.x = x;
    }
}

fn snake_color() -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=7)
}
