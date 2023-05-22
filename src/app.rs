use rand::{thread_rng, Rng};

use std::collections::VecDeque;
use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct SnakeHead {
    pub coordinates: (u16, u16),
    pub direction: Direction,
}

impl Default for SnakeHead {
    fn default() -> Self {
        Self {
            coordinates: (5, 5),
            direction: Direction::RIGHT,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub running: bool,

    pub game_over: bool,

    pub head: SnakeHead,

    pub body: VecDeque<(u16, u16)>,

    pub prey: (u16, u16),

    pub boundaries: (u16, u16),

    pub score: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game_over: false,
            boundaries: (0, 0),
            head: SnakeHead::default(),
            body: {
                let mut buf = VecDeque::new();
                buf.push_back((4, 5));
                buf.push_back((3, 5));
                buf
            },
            prey: Self::generate_prey(10, 10),
            score: 0,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_prey(x: u16, y: u16) -> (u16, u16) {
        let mut rng = thread_rng();
        (rng.gen_range(2..x), rng.gen_range(2..y))
    }

    pub fn tick(&mut self) {
        if self.body.contains(&self.head.coordinates) {
            self.game_over = true;
        }

        if self.head.coordinates == self.prey {
            let back = self.body.pop_back().unwrap();
            let new_back = {
                if back.0 > 0 {
                    (back.0 - 1, back.1)
                } else if back.1 > 0 {
                    (back.0, back.1 - 1)
                } else {
                    (0, 0)
                }
            };
            self.body.push_back(back);
            self.body.push_back(new_back);

            self.prey = Self::generate_prey(self.boundaries.0, self.boundaries.1);
            self.score += 1;
        }

        self.body.push_front(self.head.coordinates);
        self.body.pop_back();
        match self.head.direction {
            Direction::UP => {
                if self.head.coordinates.1 <= 1 {
                    self.game_over = true;
                    return;
                }
                self.head.coordinates.1 -= 1;
            }
            Direction::DOWN => {
                if self.head.coordinates.1 >= self.boundaries.1 {
                    self.game_over = true;
                    return;
                }
                self.head.coordinates.1 += 1;
            }
            Direction::LEFT => {
                if self.head.coordinates.0 <= 1 {
                    self.game_over = true;
                    return;
                }
                self.head.coordinates.0 -= 1;
            }
            Direction::RIGHT => {
                if self.head.coordinates.0 >= self.boundaries.0 {
                    self.game_over = true;
                    return;
                }
                self.head.coordinates.0 += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
