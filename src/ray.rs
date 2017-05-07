use vec3::*;

#[derive(PartialEq,Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(p: Vec3, o: Vec3) -> Ray {
        Ray {
            origin: p,
            direction: o.normalize(),
        }
    }

    pub fn traverse(&self, t: f64) -> Vec3 {
        &self.origin + &(t*&self.direction)
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
