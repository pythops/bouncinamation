use std::error;
use tui::layout::Rect;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub coordinates: (u16, u16),
    pub move_left: bool,
    pub move_top: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            coordinates: (1, 1),
            move_left: false,
            move_top: false,
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
            self.move_left = false
        }

        if y == 0 {
            self.move_top = false
        }

        // 31 is the logo width
        if x >= term_size.width - 31 {
            self.move_left = true;
        }

        if y >= term_size.height - 9 {
            self.move_top = true
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
