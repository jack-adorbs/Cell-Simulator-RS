use super::vec2::Vec2;


pub struct Object {
    pub size: i16,
    pub location: Vec2<i16>,
}

impl Object {
    pub fn new(size: i16, location: Vec2<i16>) -> Self {
        Self {
            size,
            location
        }
    }

    pub fn collision(&self, other: &Object) -> bool {
        let s2 = Vec2::new(
            self.location.x+self.size, 
            self.location.y+self.size
        );

        let o2 = Vec2::new(
            other.location.x+other.size, 
            other.location.y+other.size
        );

        
        false
    }
}