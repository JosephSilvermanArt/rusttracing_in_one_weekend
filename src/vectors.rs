use rand::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::convert::Into;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T>
where
    T: Send + Sync + Add + Mul + Sub,
{
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Vector3<T>
where
    T: Send
        + Sync
        + Add<Output = T>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Into<f64>
        + From<f64>
        + Into<T>,
{
    pub fn div(&self, o: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x / o.x,
            y: self.y / o.y,
            z: self.z / o.z,
        }
    }
    pub fn from_tuple(t: (T, T, T)) -> Vector3<T> {
        Vector3 {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
    pub fn from_vector(t: Vec<T>) -> Vector3<T> {
        Vector3 {
            x: t[0],
            y: t[1],
            z: t[2],
        }
    }
    pub fn random_in_unit_disk() -> Vector3<f64> {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("TIME FAILED");
        let mut rng = StdRng::from_entropy();

        loop {
            let p = Vector3::from_tuple((rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0));
            if p.sqrmagnitude() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vector3<f64> {
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("TIME FAILED");
        let mut rng = StdRng::from_entropy();
        let a = rng.gen_range(0.0, std::f64::consts::PI * 2.0);
        let z = rng.gen_range(-1.0, 1.0);
        let r = ((1.0 - (z * z)) as f64).sqrt();
        return Vector3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z: z,
        };
    }
    pub fn random_in_hemisphere(n: Vector3<f64>) -> Vector3<f64> {
        let in_unit_sphere = Vector3::random_in_unitsphere();
        match in_unit_sphere.dot(&n) > 0.0 // In the same hemisphere as the normal
        {    true => in_unit_sphere,
            false => &in_unit_sphere * -1.0}
    }
    pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
        let refl = &n * (2.0 * v.dot(&n));
        return v - refl;
    }
    pub fn refract(v: Vector3<f64>, n: Vector3<f64>, index: f64) -> Vector3<f64> {
        let cos_theta = (&v * -1.0).dot(&n);
        let out_perp = &(v + cos_theta * n) * index;
        let out_par = ((1.0 - out_perp.sqrmagnitude()).abs().sqrt() * -1.0) * n;
        return out_perp + out_par;
    }
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
    pub fn inf() -> Vector3<T> {
        Vector3 {
            x: (f64::INFINITY).into(),
            y: (f64::INFINITY).into(),
            z: (f64::INFINITY).into(),
        }
    }
    pub fn neg_inf() -> Vector3<T> {
        Vector3 {
            x: (f64::INFINITY).into(),
            y: (f64::INFINITY).into(),
            z: (f64::INFINITY).into(),
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
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("TIME FAILED");
        let mut rng = StdRng::from_entropy();

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
            x: (0.0).into(),
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
    T: Send
        + Sync
        + Add<Output = T>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + From<f64>,
    U: Send + Sync + Into<T>,
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
    T: Send
        + Sync
        + Add<Output = T>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Into<f64>,
    U: Send + Sync + Into<T>,
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
//RHS DIV
impl<T> Div<&Vector3<T>> for f64
where
    T: Send
        + Sync
        + Add<Output = T>
        + Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = f64>
        + Into<f64>,
{
    type Output = Vector3<f64>;
    fn div(self: f64, other: &Vector3<T>) -> Vector3<f64> {
        // let o: f64 = other.from();
        let v: Vector3<f64> = Vector3 {
            x: self / other.x.into(),
            y: self / other.y.into(),
            z: self / other.z.into(),
        };
        return v;
    }
}

impl Sum<Vector3<f64>> for Vector3<f64> {
    fn sum<I>(iter: I) -> Vector3<f64>
    where
        I: Iterator<Item = Vector3<f64>>,
    {
        iter.fold(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            |a, b| Vector3 {
                x: a.x + b.x,
                y: a.y + b.y,
                z: a.z + b.z,
            },
        )
    }
}
impl<'a> Sum<&'a Vector3<f64>> for Vector3<f64> {
    fn sum<I>(iter: I) -> Vector3<f64>
    where
        I: Iterator<Item = &'a Vector3<f64>>,
    {
        iter.fold(
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            |a, b| Vector3 {
                x: a.x + b.x,
                y: a.y + b.y,
                z: a.z + b.z,
            },
        )
    }
}
impl<'a, T> Add<&'a Vector3<T>> for &'a Vector3<T>
where
    T: Send + Sync + Add<Output = T> + Mul + Sub + Copy + Into<f64>,
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
    T: Send + Sync + Add<Output = T> + Mul + Sub + Copy + Into<f64>,
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
    T: Send + Sync + Add<Output = T> + Mul + Sub<Output = T> + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn sub(self, other: &Vector3<T>) -> Vector3<T> {
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
    T: Send + Sync + Add<Output = T> + Mul + Sub<Output = T> + Copy + Into<f64>,
{
    type Output = Vector3<T>;
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
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
    T: Send + Sync + Mul<Output = T> + Add + Copy + Sub + From<U>,
    U: Send + Sync + Into<T>,
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
impl<'a, 'b, U, T> Mul<&'a Vector3<U>> for Vector3<T>
where
    //TODO: Can i get rid of copy here?
    T: Send + Sync + Mul<Output = T> + Add + Copy + Sub,
    U: Send + Sync + Into<T> + Add + Mul + Sub + Copy,
{
    type Output = Vector3<T>;
    fn mul(self, other: &'a Vector3<U>) -> Vector3<T> {
        Vector3 {
            x: self.x * other.x.into(),
            y: self.y * other.y.into(),
            z: self.z * other.z.into(),
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
    T: Send + Sync + Display + Add + Mul + Sub,
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
