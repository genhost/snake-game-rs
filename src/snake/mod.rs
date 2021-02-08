#![allow(dead_code)]

use crate::color;
use crate::constants::DEFAULT_SNAKE_DIR;
use crate::Food;
use pancurses::{Window, COLOR_PAIR};

// pub mod bot;
pub mod player;

pub trait Snake<'a> {
    // starts the snake moving, redrawing itself etc
    // should run in a common loop
    fn start(&mut self, food: &mut Food) {
        if !self.alive() {
            self.death();
            return;
        }
        self.ctrls();
        self.mv();
        self.draw();
        self.eaten(food);
    }

    // checks snake alive
    fn alive(&mut self) -> bool {
        if self.lp().y <= self.screen().get_beg_y() - 1
            || self.lp().x <= self.screen().get_beg_x() - 1
            || self.lp().y >= self.screen().get_max_y()
            || self.lp().x >= self.screen().get_max_x()
        {
            return false;
        } else {
            let mut lp = self.lp().clone();
            self.lpmv(&mut lp);
            for part in self.pwl() {
                if lp.cmp(&part) {
                    return false;
                }
            }
        }
        true
    }

    // moves snake to smth direction
    fn mv(&mut self) {
        let mut lead_part = self.lp();
        self.lpmv(&mut lead_part);
        for part in &mut self.pwl() {
            match part.dir {
                SnakeDir::Up => part.mv(part.y - 1, part.x),
                SnakeDir::Down => part.mv(part.y + 1, part.x),
                SnakeDir::Left => part.mv(part.y, part.x - 1),
                SnakeDir::Right => part.mv(part.y, part.x + 1),
            }
        }
        self.parts_mut().push(lead_part);

        let mut parts = self.parts().clone();
        parts.reverse();
        self.parts_mut().clear();

        for i in 0..self.len() {
            self.parts_mut()
                .push(SnakePart::new(parts[i].y, parts[i].x))
        }
        self.parts_mut().reverse();
    }

    // redraws all game data on the screen
    fn draw(&mut self) {
        for part in self.parts() {
            self.screen().mvaddch(part.y, part.x + 1, ' ');
            self.screen().mvaddch(part.y, part.x - 1, ' ');
            self.screen().mvaddch(part.y + 1, part.x, ' ');
            self.screen().mvaddch(part.y - 1, part.x, ' ');
        }

        for part in self.parts() {
            let color = match self.color() {
                9 => color::rand_color(),
                _ => self.color(),
            };

            self.screen().attron(COLOR_PAIR(color));
            self.screen().mvaddch(part.y, part.x, self.form());
            self.screen().attroff(COLOR_PAIR(color));
        }
    }

    // implements direction management by player
    fn ctrls(&mut self);

    // checks snake ate smth
    fn eaten(&mut self, food: &mut Food) {
        if self.lp().y == food.y && self.lp().x == food.x {
            food.rand_move();
            self.inc_size(self.incval());
            self.score_inc(1);
        }
    }

    fn death(&mut self) {
        for part in self.parts() {
            self.screen().mvaddch(part.y, part.x, ' ');
        }
    }

    // gives leader (last) part
    fn lp(&self) -> SnakePart {
        self.parts()[self.parts().len() - 1]
    }

    // moves leader part
    fn lpmv(&mut self, lp: &mut SnakePart) {
        let offset = 1;
        match self.dir() {
            SnakeDir::Up => lp.mv(lp.y - offset, lp.x),
            SnakeDir::Down => lp.mv(lp.y + offset, lp.x),
            SnakeDir::Left => lp.mv(lp.y, lp.x - offset),
            SnakeDir::Right => lp.mv(lp.y, lp.x + offset),
        }
    }

    // gives parts without leader part
    fn pwl(&self) -> Vec<SnakePart> {
        self.parts()[..self.parts().len()].to_vec()
    }

    // increases snake size
    fn inc_size(&mut self, size: usize) {
        for _i in 0..size {
            self.parts_mut().reverse();
            let part = self.parts_mut()[0].clone();
            self.parts_mut().reverse();
            self.parts_mut().push(part);
            self.len_inc(1);
        }
    }

    // getters
    fn len(&self) -> usize;
    fn color(&self) -> u32;
    fn form(&self) -> char;
    fn score(&self) -> usize;
    fn incval(&self) -> usize;
    fn dir(&self) -> &SnakeDir;
    fn screen(&self) -> &Window;
    fn parts(&self) -> &Vec<SnakePart>;

    // setters
    fn len_inc(&mut self, n: usize);
    fn score_inc(&mut self, n: usize);
    fn parts_mut(&mut self) -> &mut Vec<SnakePart>;
}

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
pub enum SnakeDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct SnakePart {
    x: i32,
    y: i32,
    dir: SnakeDir,
}

impl SnakePart {
    pub fn dnew(y: i32, x: i32, dir: SnakeDir) -> Self {
        Self { x, y, dir }
    }

    fn new(y: i32, x: i32) -> Self {
        Self {
            x,
            y,
            dir: DEFAULT_SNAKE_DIR,
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
