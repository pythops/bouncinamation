use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::error;
use tui::{layout::Rect, style::Color};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum LogoColor {
    White,
    Red,
    Blue,
    Cyan,
    Yellow,
    Magenta,
    Green,
    Gray,
}

impl From<LogoColor> for Color {
    fn from(color: LogoColor) -> Self {
        match color {
            LogoColor::White => Color::White,
            LogoColor::Red => Color::Red,
            LogoColor::Blue => Color::Blue,
            LogoColor::Cyan => Color::Cyan,
            LogoColor::Yellow => Color::Yellow,
            LogoColor::Magenta => Color::Magenta,
            LogoColor::Green => Color::Green,
            LogoColor::Gray => Color::Gray,
        }
    }
}

impl Distribution<LogoColor> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LogoColor {
        match rng.gen_range(0..=8) {
            0 => LogoColor::White,
            1 => LogoColor::Red,
            2 => LogoColor::Blue,
            3 => LogoColor::Cyan,
            4 => LogoColor::Yellow,
            5 => LogoColor::Magenta,
            6 => LogoColor::Green,
            7 => LogoColor::Gray,
            _ => LogoColor::White,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub coordinates: (u16, u16),
    pub move_left: bool,
    pub move_top: bool,
    pub color: Color,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            coordinates: (1, 1),
            move_left: false,
            move_top: false,
            color: Color::White,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, term_size: Rect) {
        let mut x = self.coordinates.0;
        let mut y = self.coordinates.1;

        if x == 0 {
            self.move_left = false;
            self.color = rand::random::<LogoColor>().into();
        }

        if y == 0 {
            self.move_top = false;
            self.color = rand::random::<LogoColor>().into();
        }

        // 31 is the logo width
        if x >= term_size.width - 31 {
            self.move_left = true;
            self.color = rand::random::<LogoColor>().into();
        }

        if y >= term_size.height - 9 {
            self.move_top = true;
            self.color = rand::random::<LogoColor>().into();
        }

        if self.move_left {
            x -= 1;
        } else {
            x += 1;
        }

        if self.move_top {
            y -= 1;
        } else {
            y += 1;
        }

        self.coordinates = (x, y);
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn step(&mut self) {}
}
