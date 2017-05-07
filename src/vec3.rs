use std::ops::{Add, Sub, Mul, Div};
use std::convert::From;

#[derive(Copy,Clone,PartialEq,Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl<'a> Vec3 {
    pub fn unit() -> Vec3 {
        Vec3 { x: 1., y: 1., z: 1., }
    }

    pub fn zero() -> Vec3 {
        Vec3 {x: 0., y: 0., z: 0.}
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.x*rhs.z - self.z*rhs.x,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.magnitude()
    }
}

impl From<(u8,u8,u8)> for Vec3 {
    fn from(triple: (u8,u8,u8)) -> Vec3 {
        let (x, y, z) = triple;
        Vec3 {
            x: f64::from(x),
            y: f64::from(y),
            z: f64::from(z),
        }
    }
}

impl<'a> Add<f64> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<'a> Add<&'a Vec3> for f64 {
    type Output = Vec3;
    fn add(self, rhs: &'a Vec3) -> Self::Output {
        Vec3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl<'a,'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> Sub<f64> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<'a> Sub<&'a Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, rhs: &'a Vec3) -> Self::Output {
        Vec3 {
            x: self - rhs.x,
            y: self - rhs.y,
            z: self - rhs.z,
        }
    }
}

impl<'a,'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<'a> Mul<&'a Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<'a> Div<&'a Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: &'a Vec3) -> Self::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
