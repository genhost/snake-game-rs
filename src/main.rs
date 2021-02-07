#![allow(dead_code)]

use clap::{App, Arg};
use pancurses::*;
use rand::{thread_rng, Rng};
use std::convert::TryInto;

static DEFAULT_SNAKE_DIR: SnakeDir = SnakeDir::Right;
static DEFAULT_SNAKE_COLOR: i16 = COLOR_PAIR_GREEN;
static DEFAULT_SNAKE_SIZE: usize = 10;
static DEFAULT_PART_CHAR: char = '#';
static DEFAULT_APPLE_CHAR: char = '#';
static DEFAULT_APPLE_COLOR: i16 = COLOR_PAIR_RED;
static DEFAULT_APPLE_INCSIZE: usize = 5;
static DEFAULT_DISPLAY_STATE: &str = "points";

// Don't change these constants
static COLOR_PAIR_WHITE: i16 = 1;
static COLOR_PAIR_BLUE: i16 = 2;
static COLOR_PAIR_CYAN: i16 = 3;
static COLOR_PAIR_GREEN: i16 = 4;
static COLOR_PAIR_RED: i16 = 5;
static COLOR_PAIR_YELLOW: i16 = 6;
static COLOR_PAIR_MAGENTA: i16 = 7;
static COLOR_PAIR_BLACK: i16 = 8;
static COLOR_RANDOM: i16 = 9;

fn main() {
    let matches = App::new("Snake Game")
        .author("General Host")
        .about("Simple snake game written in Rust using pancurses")
        .arg(
            Arg::with_name("snake char")
                .long("snake-char")
                .value_name("CHAR")
                .help(&format!("Sets snake char. Default is {}", DEFAULT_PART_CHAR)),
        )
        .arg(
            Arg::with_name("snake size")
                .long("snake-size")
                .value_name("NUMBER")
                .help(&format!("Sets initial snake size. Default is {}", DEFAULT_SNAKE_SIZE)),
        )
        .arg(
            Arg::with_name("snake direction")
                .long("snake-dir")
                .value_name("up | down | left | right")
                .help("Sets initial snake direction. Default is right"),
        )
        .arg(
            Arg::with_name("snake color")
                .long("snake-color")
                .value_name("red | white | magenta | cyan | black | green | blue | yellow")
                .help("Sets initial snake color. Default is green"),
        )
        .arg(
            Arg::with_name("apple increase size")
                .long("apple-inc")
                .value_name("NUMBER")
                .help(&format!("Sets how much the snake will increase in size after it eats an apple. Default is {}", DEFAULT_APPLE_INCSIZE)),
        )
        .arg(
            Arg::with_name("apple char")
                .long("apple-char")
                .value_name("CHAR")
                .help(&format!("Sets apple char. Default is {}", DEFAULT_APPLE_CHAR)),
        )
        .arg(
            Arg::with_name("apple color")
                .long("apple-color")
                .value_name("red | white | magenta | cyan | black | green | blue | yellow | random")
                .help("Sets apple color. Default is red"),
        )
        .arg(
            Arg::with_name("hardcore")
                .short("h")
                .multiple(true)
                .help("If you are bored with the usual snake, this flag is for you. The more you add this flag (-hh, -hhh etc), the more difficult it will be to play, to the point of being unplayable. Note: Game with this flag is being very unstable"),
        )
        .arg(
            Arg::with_name("display")
                .long("display")
                .value_name("none | points")
                .help("Change what to display at the game time. Default is points"),
        )
        .get_matches();

    let snake_char = matches
        .value_of("snake char")
        .unwrap_or(&String::from(DEFAULT_PART_CHAR))
        .chars()
        .nth(0)
        .unwrap();
    let snake_size = matches
        .value_of("snake size")
        .unwrap_or(&DEFAULT_SNAKE_SIZE.to_string())
        .parse::<usize>()
        .unwrap();
    let snake_dir = match matches.value_of("snake direction") {
        Some("up") => SnakeDir::Up,
        Some("down") => SnakeDir::Down,
        Some("left") => SnakeDir::Left,
        Some("right") => SnakeDir::Right,
        _ => DEFAULT_SNAKE_DIR,
    };
    let snake_color = match matches.value_of("snake color") {
        Some("white") => COLOR_PAIR_WHITE,
        Some("blue") => COLOR_PAIR_BLUE,
        Some("cyan") => COLOR_PAIR_CYAN,
        Some("black") => COLOR_PAIR_BLACK,
        Some("red") => COLOR_PAIR_RED,
        Some("green") => COLOR_PAIR_GREEN,
        Some("yellow") => COLOR_PAIR_YELLOW,
        Some("magenta") => COLOR_PAIR_MAGENTA,
        Some("random") => COLOR_RANDOM,
        _ => DEFAULT_SNAKE_COLOR,
    };
    let apple_char = matches
        .value_of("apple char")
        .unwrap_or(&String::from(DEFAULT_APPLE_CHAR))
        .chars()
        .nth(0)
        .unwrap();
    let apple_incsize = matches
        .value_of("apple increase size")
        .unwrap_or(&DEFAULT_APPLE_INCSIZE.to_string())
        .parse::<usize>()
        .unwrap();
    let apple_color = match matches.value_of("apple color") {
        Some("white") => COLOR_PAIR_WHITE,
        Some("blue") => COLOR_PAIR_BLUE,
        Some("cyan") => COLOR_PAIR_CYAN,
        Some("black") => COLOR_PAIR_BLACK,
        Some("red") => COLOR_PAIR_RED,
        Some("green") => COLOR_PAIR_GREEN,
        Some("yellow") => COLOR_PAIR_YELLOW,
        Some("magenta") => COLOR_PAIR_MAGENTA,
        Some("random") => COLOR_RANDOM,
        _ => DEFAULT_APPLE_COLOR,
    };
    let hardcore_level = matches.occurrences_of("hardcore") as i32;
    let display = matches.value_of("display").unwrap_or(DEFAULT_DISPLAY_STATE);

    let window = initscr();

    start_color();

    init_pair(COLOR_PAIR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
    init_pair(COLOR_PAIR_CYAN, COLOR_CYAN, COLOR_BLACK);
    init_pair(COLOR_PAIR_RED, COLOR_RED, COLOR_BLACK);
    init_pair(COLOR_PAIR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(COLOR_PAIR_WHITE, COLOR_WHITE, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLUE, COLOR_BLUE, COLOR_BLACK);
    init_pair(COLOR_PAIR_GREEN, COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_PAIR_BLACK, COLOR_BLACK, COLOR_BLACK);

    curs_set(0);
    noecho();

    let mut snake = Snake::new(
        &window,
        display,
        snake_char,
        snake_size,
        apple_incsize,
        snake_dir,
        snake_color.try_into().unwrap(),
        Apple::new(&window, apple_char, apple_color),
        hardcore_level,
    );

    snake.start();

    endwin();
}

struct Snake<'a> {
    form: char,
    color: u32,
    dir: SnakeDir,
    length: usize,
    hardcore: i32,
    points: usize,
    inc_size: usize,
    display: &'a str,
    apple: Apple<'a>,
    window: &'a Window,
    parts: Vec<SnakePart>,
}

impl<'a> Snake<'a> {
    fn new(
        window: &'a Window,
        display: &'a str,
        form: char,
        size: usize,
        inc_size: usize,
        dir: SnakeDir,
        color: u32,
        apple: Apple<'a>,
        hardcore: i32,
    ) -> Self {
        let mut parts: Vec<SnakePart> = Vec::new();
        let window_center = (window.get_max_y() / 2, window.get_max_x() / 2);
        for x_offset in 0..size {
            parts.push(SnakePart::new(
                window_center.0,
                window_center.1 + x_offset as i32,
            ))
        }
        if let SnakeDir::Left = dir {
            parts.reverse()
        }
        Self {
            dir,
            form,
            parts,
            window,
            apple,
            display,
            points: 0,
            color,
            inc_size,
            hardcore,
            length: size,
        }
    }

    fn sync(&mut self) {
        let mut parts = self.parts.clone();
        parts.reverse();
        self.parts.clear();

        for i in 0..self.length {
            self.parts.push(SnakePart::new(parts[i].y, parts[i].x))
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

            self.check_eaten();
            self.mv();
            self.redraw();
        }
    }

    fn last_part_mv(&mut self, last_part: &mut SnakePart) {
        let offset;
        if self.hardcore > 0 {
            offset = self.hardcore;
        } else {
            offset = 1;
        }
        match self.dir {
            SnakeDir::Up => last_part.mv(last_part.y - offset, last_part.x),
            SnakeDir::Down => last_part.mv(last_part.y + offset, last_part.x),
            SnakeDir::Left => last_part.mv(last_part.y, last_part.x - offset),
            SnakeDir::Right => last_part.mv(last_part.y, last_part.x + offset),
        }
    }

    fn mv(&mut self) {
        let mut last_part = self.last_part();
        self.last_part_mv(&mut last_part);
        for part in &mut self.parts_without_last() {
            match part.dir {
                SnakeDir::Up => part.mv(part.y - 1, part.x),
                SnakeDir::Down => part.mv(part.y + 1, part.x),
                SnakeDir::Left => part.mv(part.y, part.x - 1),
                SnakeDir::Right => part.mv(part.y, part.x + 1),
            }
        }
        self.parts.push(last_part);
    }

    fn check_game_over(&mut self) -> bool {
        if self.last_part().y <= self.window.get_beg_y() - 1
            || self.last_part().x <= self.window.get_beg_x() - 1
            || self.last_part().y >= self.window.get_max_y()
            || self.last_part().x >= self.window.get_max_x()
        {
            return true;
        } else {
            let mut last_part = self.last_part().clone();
            self.last_part_mv(&mut last_part);
            for part in self.parts_without_last() {
                if last_part.cmp(&part) {
                    return true;
                }
            }
        }
        false
    }

    fn last_part(&self) -> SnakePart {
        self.parts[self.parts.len() - 1]
    }

    fn parts_without_last(&self) -> Vec<SnakePart> {
        self.parts[..self.parts.len()].to_vec()
    }

    fn redraw(&self) {
        // std::thread::sleep(std::time::Duration::from_millis(0));
        self.window.clear();
        self.apple.spawn();

        if let "points" = self.display {
            self.window.mvaddstr(self.window.get_beg_y(),
                self.window.get_beg_x(),
                &format!("Points: {}\nLength: {}", self.points, self.length)
            );
        }

        for part in &self.parts {
            let color = match self.color {
                9 => rand_color(),
                _ => self.color,
            };

            self.window.attron(COLOR_PAIR(color));
            self.window.mvaddch(part.y, part.x, self.form);
            self.window.attroff(COLOR_PAIR(color));

            self.window.refresh();
        }
    }

    fn check_eaten(&mut self) {
        if self.last_part().y == self.apple.y && self.last_part().x == self.apple.x {
            self.apple.rand_move();
            self.inc_size(self.inc_size);
            self.points += 1;
        }
    }

    fn inc_size(&mut self, size: usize) {
        for _i in 0..size {
            self.parts.reverse();
            let part = self.parts[0].clone();
            self.parts.reverse();
            self.parts.push(part);
            self.length += 1;
        }
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

    fn cmp(&self, with: &Self) -> bool {
        if self.y == with.y && self.x == with.x {
            return true;
        }
        false
    }
}

fn rand_color() -> u32 {
    thread_rng().gen_range(1..=7)
}

struct Apple<'a> {
    x: i32,
    y: i32,
    form: char,
    color: i16,
    window: &'a Window,
}

impl<'a> Apple<'a> {
    fn new(window: &'a Window, form: char, color: i16) -> Self {
        Self {
            x: thread_rng().gen_range(window.get_beg_x()..=window.get_max_x()),
            y: thread_rng().gen_range(window.get_beg_y()..=window.get_max_y()),
            form,
            color,
            window,
        }
    }

    fn rand_move(&mut self) {
        self.x = thread_rng().gen_range(self.window.get_beg_x()..=self.window.get_max_x());
        self.y = thread_rng().gen_range(self.window.get_beg_y()..=self.window.get_max_y());
    }

    fn spawn(&self) {
        self.window
            .attron(COLOR_PAIR(self.color.try_into().unwrap()));
        self.window.mvaddch(self.y, self.x, self.form);
        self.window
            .attroff(COLOR_PAIR(self.color.try_into().unwrap()));
    }
}
