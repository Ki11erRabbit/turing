pub mod ast;
pub mod parser;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub enum Direction {
    Left,
    Right,
}

pub trait Tape<T>: std::fmt::Debug {
    fn get(&self, index: usize) -> T;
    fn set(&mut self, index: usize, value: T);
    fn add(&mut self, index: usize, direction: Direction, offset: Option<usize>);
    fn sub(&mut self, index: usize, direction: Direction, offset: Option<usize>);
    fn mul(&mut self, index: usize, direction: Direction, offset: Option<usize>);
    fn div(&mut self, index: usize, direction: Direction, offset: Option<usize>);
    fn modulo(&mut self, index: usize, direction: Direction, offset: Option<usize>);
    fn grow(&mut self);
    fn in_bounds(&self, index: usize) -> bool;
}

pub trait Number: Copy + Clone + PartialEq + PartialOrd + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> + std::ops::Rem<Output = Self> + Default + std::fmt::Debug + std::fmt::Display {
    fn is_zero(&self) -> bool;
    fn is_nonzero(&self) -> bool;
    fn from(i: i64) -> Self;
    fn to_u64(&self) -> u64;
}

impl Number for i64 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0
    }

    fn from(i: i64) -> Self {
        i
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl Number for i32 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0
    }

    fn from(i: i64) -> Self {
        i as i32
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl Number for i16 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0
    }

    fn from(i: i64) -> Self {
        i as i16
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl Number for i8 {
    fn is_zero(&self) -> bool {
        *self == 0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0
    }

    fn from(i: i64) -> Self {
        i as i8
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl Number for f64 {
    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0.0
    }

    fn from(i: i64) -> Self {
        i as f64
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}
impl Number for f32 {
    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    fn is_nonzero(&self) -> bool {
        *self != 0.0
    }

    fn from(i: i64) -> Self {
        i as f32
    }

    fn to_u64(&self) -> u64 {
        *self as u64
    }
}


impl<T: Number> Tape<T> for Vec<T> {
    fn get(&self, index: usize) -> T {
        self[index]
    }

    fn set(&mut self, index: usize, value: T) {
        self[index] = value;
    }

    fn add(&mut self, index: usize, direction: Direction, offset: Option<usize>) {
        let offset = offset.unwrap_or(1);
        match direction {
            Direction::Left => {
                self[index - offset] = self[index - offset] + self[index];
            }
            Direction::Right => {
                while index + offset >= self.len() {
                    self.grow();
                }
                self[index + offset] = self[index + offset] + self[index];
            }
        }
    }

    fn sub(&mut self, index: usize, direction: Direction, offset: Option<usize>) {
        let offset = offset.unwrap_or(1);
        match direction {
            Direction::Left => {
                self[index - offset] = self[index - offset] - self[index];
            }
            Direction::Right => {
                self[index + offset] = self[index + offset] - self[index];
            }
        }
    }

    fn mul(&mut self, index: usize, direction: Direction, offset: Option<usize>) {
        let offset = offset.unwrap_or(1);
        match direction {
            Direction::Left => {
                self[index - offset] = self[index - offset] * self[index];
            }
            Direction::Right => {
                self[index + offset] = self[index + offset] * self[index];
            }
        }
    }

    fn div(&mut self, index: usize, direction: Direction, offset: Option<usize>) {
        let offset = offset.unwrap_or(1);
        match direction {
            Direction::Left => {
                self[index - offset] = self[index - offset] / self[index];
            }
            Direction::Right => {
                self[index + offset] = self[index + offset] / self[index];
            }
        }
    }

    fn modulo(&mut self, index: usize, direction: Direction, offset: Option<usize>) {
        let offset = offset.unwrap_or(1);
        match direction {
            Direction::Left => {
                self[index - offset] = self[index - offset] % self[index];
            }
            Direction::Right => {
                self[index + offset] = self[index + offset] % self[index];
            }
        }
    }

    fn grow(&mut self) {
        self.push(T::default());
    }

    fn in_bounds(&self, index: usize) -> bool {
        index < self.len()
    }
}
