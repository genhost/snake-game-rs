use pancurses::*;
use crate::constants::*;
use crate::food::Food;
use clap::{App, Arg};
use crate::snake::{Snake, SnakeDir, SnakePart, player::SnakePlayer};

mod color;
mod constants;
mod food;
mod snake;

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
                .value_name("red | white | magenta | cyan | black | green | blue | yellow | random")
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
        // .arg(
        //     Arg::with_name("hardcore")
        //         .short("h")
        //         .multiple(true)
        //         .help("If you are bored with the usual snake, this flag is for you. The more you add this flag (-hh, -hhh etc), the more difficult it will be to play, to the point of being unplayable. Note: Game with this flag is being very unstable"),
        // )
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
    // let hardcore_level = matches.occurrences_of("hardcore") as i32;
    let display = matches.value_of("display").unwrap_or(DEFAULT_DISPLAY_STATE);

    let window = initscr();

    let mut food = Food::new(&window, apple_char, apple_color);
    
    let mut snake = SnakePlayer::new(
        &window,
        snake_char,
        snake_size,
        apple_incsize,
        SnakePart::dnew(COORD_CENTER, COORD_CENTER, snake_dir),
        snake_color as u32,
    );
    
    noecho();
    curs_set(0);
    window.keypad(true);

    color::init();

    while snake.alive() {
        if let "points" = display {
            window.mvaddstr(
                window.get_beg_y(),
                window.get_beg_x(),
                &format!("Score: {}\nLength: {}", snake.score(), snake.len()),
            );
        }

        snake.start(&mut food);
        food.spawn();

    }
    
    endwin();
}
