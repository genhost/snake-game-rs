use crate::color;
use pancurses::{Window, COLOR_PAIR};
use rand::{thread_rng, Rng};

pub struct Food<'a> {
    pub x: i32,
    pub y: i32,
    form: char,
    color: i16,
    window: &'a Window,
}

impl<'a> Food<'a> {
    pub fn new(window: &'a Window, form: char, color: i16) -> Self {
        Self {
            x: thread_rng().gen_range(window.get_beg_x()..=window.get_max_x()),
            y: thread_rng().gen_range(window.get_beg_y()..=window.get_max_y()),
            form,
            color,
            window,
        }
    }

    pub fn rand_move(&mut self) {
        self.x = thread_rng().gen_range(self.window.get_beg_x()..=self.window.get_max_x());
        self.y = thread_rng().gen_range(self.window.get_beg_y()..=self.window.get_max_y());
    }

    pub fn spawn(&self) {
        let color = match self.color {
            9 => color::rand_color(),
            _ => self.color as u32,
        };
        self.window.attron(COLOR_PAIR(color));
        self.window.mvaddch(self.y, self.x, self.form);
        self.window.attroff(COLOR_PAIR(color));
    }
}