use num::Integer;

use std::{num::TryFromIntError, ops::{Add, AddAssign}};

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> 
where T: Integer {
    pub fn new(x: T, y: T) -> Self {
        Self { 
            x, 
            y 
        }
    }
}

impl<T> Add for Vec2<T>
where T: Integer {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self::new(
            self.x + rhs.x, 
            self.y + rhs.y
        )
    }
}
impl<T> AddAssign for Vec2<T> 
where T: Integer + AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;

    }    
}

impl TryFrom<Vec2<i32>> for Vec2<i16> {
    type Error = TryFromIntError;

    fn try_from(value: Vec2<i32>) -> Result<Self, Self::Error> {
        let x: Result<i16, Self::Error> = value.x.try_into();
        let y: Result<i16, Self::Error> = value.y.try_into();

        match (x, y) {
            (Ok(x), Ok(y)) => Ok(Self::new(x, y)),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(_), Err(e)) => Err(e),
        }
    }
}

impl From<Vec2<i16>> for Vec2<i32> {
    fn from(value: Vec2<i16>) -> Self {
        Self::new(value.x.into(), value.y.into())
    }
}