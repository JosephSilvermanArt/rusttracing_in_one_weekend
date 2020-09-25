use rand::prelude::*;
use std::convert::Into;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T>
where
    T: Add + Mul + Sub,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Vector3<T>
where
    T: Add<Output = T>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Into<f64>
        + From<f64>
        + Into<T>,
{
    pub fn random_in_unitsphere() -> Vector3<T> {
        loop {
            let p = Vector3::random_range(-1.0, 1.0);
            if p.sqrmagnitude() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn zero() -> Vector3<T> {
        Vector3 {
            x: (0.0).into(),
            y: (0.0).into(),
            z: (0.0).into(),
        }
    }
    pub fn random() -> Vector3<T> {
        Vector3 {
            x: thread_rng().gen::<f64>().into(),
            y: thread_rng().gen::<f64>().into(),
            z: thread_rng().gen::<f64>().into(),
        }
    }
    pub fn random_range(min: f64, max: f64) -> Vector3<T> {
        let mut rng = thread_rng();
        Vector3 {
            x: rng.gen_range(min, max).into(),
            y: rng.gen_range(min, max).into(),
            z: rng.gen_range(min, max).into(),
        }
    }
    pub fn one() -> Vector3<T> {
        Vector3 {
            x: (1.0).into(),
            y: (1.0).into(),
            z: (1.0).into(),
        }
    }

    pub fn up() -> Vector3<T> {
        Vector3 {
            x: (0.0).into(),
            y: (1.0).into(),
            z: (0.0).into(),
        }
    }
    pub fn right() -> Vector3<T> {
        Vector3 {
            x: (1.0).into(),
            y: (0.0).into(),
            z: (0.0).into(),
        }
    }
    pub fn forward() -> Vector3<T> {
        Vector3 {
            x: (1.0).into(),
            y: (0.0).into(),
            z: (-1.0).into(),
        }
    }
    pub fn normalized(&self) -> Vector3<T> {
        let m = self.magnitude();
        if m == 0.0 {
            Vector3::zero()
        } else {
            self / m
        }
    }
    pub fn sqrmagnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).into()
    }
    pub fn magnitude(&self) -> f64 {
        self.sqrmagnitude().sqrt()
    }
    pub fn dot(&self, other: &Vector3<T>) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(&self, b: &Vector3<T>) -> Vector3<T> {
        let a = self;
        Vector3 {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }
}
//REF DIV
impl<'a, T, U> Div<U> for &'a Vector3<T>
where
    T: Add<Output = T> + Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64>,
    U: Into<T>,
{
    type Output = Vector3<T>;
    fn div(self, other: U) -> Vector3<T> {
        let o: T = other.into();
        Vector3 {
            x: self.x / o,
            y: self.y / o,
            z: self.z / o,
        }
    }
}

//VAL DIV
impl<T, U> Div<U> for Vector3<T>
where
    T: Add<Output = T> + Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Into<f64>,
    U: Into<T>,
{
    type Output = Vector3<T>;
    fn div(self, other: U) -> Vector3<T> {
        let o: T = other.into();
        Vector3 {
            x: self.x / o,
            y: self.y / o,
            z: self.z / o,
        }
    }
}

// OPERATOR OVERLOADING #############
//REF ADD
impl<'a, T> Add<&'a Vector3<T>> for &'a Vector3<T>
where
    T: Add<Output = T> + Mul + Sub + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn add(self, other: Self) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
//VALUE ADD
impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Add<Output = T> + Mul + Sub + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn add(self, other: Self) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
//REF SUB
impl<'a, T> Sub<&'a Vector3<T>> for &'a Vector3<T>
where
    T: Add<Output = T> + Mul + Sub<Output = T> + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn sub(self, other: Self) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
// VALUE SUB
impl<T> Sub<Vector3<T>> for Vector3<T>
where
    T: Add<Output = T> + Mul + Sub<Output = T> + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn sub(self, other: Self) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
// LHS MUL
impl<'a, T, U> Mul<U> for &'a Vector3<T>
where
    //TODO: Can i get rid of copy here?
    T: Mul<Output = T> + Add + Copy + Sub + From<U>,
    U: Into<T>,
{
    type Output = Vector3<T>;
    fn mul(self, other: U) -> Vector3<T> {
        let o: T = other.into();
        Vector3 {
            x: self.x * o,
            y: self.y * o,
            z: self.z * o,
        }
    }
}

// RHS MUL -- ONLY f64
impl<'a> Mul<Vector3<f64>> for f64
where
//TODO: Can i get rid of copy here?
{
    type Output = Vector3<f64>;
    fn mul(self, other: Vector3<f64>) -> Vector3<f64> {
        Vector3::<f64> {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<T> Display for Vector3<T>
where
    T: Display + Add + Mul + Sub,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Vector: x {}, y {}, z {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Empty_Test() {}
}
