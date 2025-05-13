use crate::physics::vec2::Vec2;
use crate::physics::object::Object;

pub struct Cell {
    pub _health: u8,
    pub _energy: u8,
    pub object: Object
}

impl Cell {
    pub fn new(_health: u8, _energy: u8, object: Object) -> Self {
        Self { 
            _health, 
            _energy, 
            object,
        }
    }

    pub fn move_by(&mut self, location: Vec2<i16>, max: (u32, u32)) {
        let mut n: Vec2<i32> = (self.object.location.clone() + location).into(); 
        let x: i32 = max.0.try_into().unwrap();
        let y: i32 = max.1.try_into().unwrap(); 

        
        if n.x < 0 {
            n.x = 0;
        }

        if n.y < 0 {
            n.y = 0;
        }

        if n.x > x {
            n.x = x;
        }

        if n.y > y {
            n.y = y;
        }

        self.object.location = n.try_into().unwrap();
    }
}
