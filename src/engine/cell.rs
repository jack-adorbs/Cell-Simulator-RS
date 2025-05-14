use crate::physics::vec2::Vec2;
use crate::physics::object::Object;

#[derive(Clone)]
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


    pub fn window_bounds_correction(object: &mut Object, max: (u32, u32)) {
        let x: i16 = max.0.try_into().unwrap();
        let y: i16 = max.1.try_into().unwrap(); 

        
        if object.location.x < 0 {
            object.location.x = 0;
        }

        if object.location.y < 0 {
            object.location.y = 0;
        }

        if object.location.x > x {
            object.location.x = x;
        }

        if object.location.y > y {
            object.location.y = y;
        }
    }

    pub fn move_by(&mut self, location: Vec2<i16>, max: (u32, u32)) {
        let n = &mut Object::new(0, (self.object.location.clone() + location).into()); 
        Cell::window_bounds_correction(n, max);
        self.object.location = n.location.clone();
    }
}
