use super::{Snake, SnakeDir, SnakePart, Window};
use crate::constants::COORD_CENTER;
use pancurses::{half_delay, Input};

pub struct SnakePlayer<'a> {
    form: char,
    color: u32,
    dir: SnakeDir,
    length: usize,
    score: usize,
    incval: usize,
    screen: &'a Window,
    parts: Vec<SnakePart>,
}

impl<'a> SnakePlayer<'a> {
    pub fn new(
        screen: &'a Window,
        form: char,
        size: usize,
        incval: usize,
        pos: SnakePart,
        color: u32,
    ) -> Self {
        let mut parts: Vec<SnakePart> = Vec::new();
        let offsets = (
            if pos.y == COORD_CENTER {
                screen.get_max_y() / 2
            } else {
                pos.y
            },
            if pos.x == COORD_CENTER {
                screen.get_max_x() / 2 - size as i32
            } else {
                pos.x
            },
        );
        for x_offset in 0..size {
            parts.push(SnakePart::new(offsets.0, offsets.1 + x_offset as i32))
        }
        if let SnakeDir::Left = pos.dir {
            parts.reverse()
        }
        Self {
            dir: pos.dir,
            form,
            parts,
            color,
            screen,
            incval,
            score: 0,
            length: size,
        }
    }
}

impl<'a> Snake<'a> for SnakePlayer<'a> {
    fn ctrls(&mut self) {
        half_delay(1);
        self.dir = match self.screen.getch() {
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
    }

    // getters
    fn form(&self) -> char {
        self.form
    }
    fn color(&self) -> u32 {
        self.color
    }
    fn len(&self) -> usize {
        self.length
    }
    fn score(&self) -> usize {
        self.score
    }
    fn dir(&self) -> &SnakeDir {
        &self.dir
    }
    fn incval(&self) -> usize {
        self.incval
    }
    fn screen(&self) -> &Window {
        self.screen
    }
    fn parts(&self) -> &Vec<SnakePart> {
        &self.parts
    }

    // setters
    fn len_inc(&mut self, n: usize) {
        self.length += n
    }
    fn score_inc(&mut self, n: usize) {
        self.score += n
    }
    fn parts_mut(&mut self) -> &mut Vec<SnakePart> {
        &mut self.parts
    }
}
