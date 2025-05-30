use super::vec2::Vec2;

#[derive(Clone)]
pub struct Object {
    pub id: u16,
    pub size: i16,
    pub location: Vec2<i16>,
}

const ID: std::cell::Cell<u16> = std::cell::Cell::new(0);


impl Object {
    pub fn new(size: i16, location: Vec2<i16>) -> Self {
        
        let s = Self {
            id: ID.get(),
            size,
            location
        };
        ID.set(ID.get() + 1);
        
        s
    }

    // TODO: Verify Correct beaviour.
    pub fn collision(&self, r2: &Object, collision_scale: f32) -> Result<bool, ()> {
        if self.id == r2.id {
            return Err(());
        }

        // (r1+r2)*c = d*c
        let d = (
            (r2.location.x - self.location.x).pow(2) + 
            (r2.location.y - self.location.y).pow(2)
        ).isqrt();
 
        Ok((((self.size + r2.size) as f32)*collision_scale) == (d as f32)*collision_scale)
    }
}