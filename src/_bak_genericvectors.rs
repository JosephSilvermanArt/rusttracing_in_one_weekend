use std::convert::Into;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
//TODO:
//1- move into a library or module or something
//2- Optimization -- understand the copies going on and whether theyre necessary. Especially concerned abt normalized
//3- Does it make sense to make a trait to collect the like 4 traits that i repeat over and over in wheres?
//4- Expand functionality as needed
//5- Why cant i just call self.clone() / v in normalize???
#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector3<T>
where
    T: Add + Mul + Sub,
{
    x: T,
    y: T,
    z: T,
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
    fn zero() -> Vector3<T> {
        Vector3 {
            x: (0.0).into(),
            y: (0.0).into(),
            z: (0.0).into(),
        }
    }
    fn normalized(&self) -> Vector3<T> {
        let m = self.magnitude();
        if m == 0.0 {
            Vector3::zero()
        } else {
            self / m
        }
    }
    fn sqrmagnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).into()
    }
    fn magnitude(&self) -> f64 {
        self.sqrmagnitude().sqrt()
    }
    fn dot(&self, other: &Vector3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    fn cross(&self, b: &Vector3) -> Vector3<T> {
        let a = self;
        Vector3 {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }
}
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

// OPERATOR OVERLOADING #############
impl<'a, T> Add<&'a Vector3<T>> for &'a Vector3<T>
where
    T: Add<Output = T> + Mul + Sub + Copy,
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
impl<'a, T, U> Mul<U> for &'a Vector3<T>
where
    //TODO: Can i get rid of copy here?
    T: Mul<Output = T> + Add + Copy + Sub,
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

impl<T> Display for Vector3<T>
where
    T: Display + Add + Mul + Sub,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Vector: x {}, y {}, z {}]", self.x, self.y, self.z)
    }
}

fn main() {
    let v1 = Vector3 { x: 10, y: 1, z: 5 };
    let v2 = v1;
    println!("Hello, world! {}", &v2 * 7);
    let v1 = Vector3 {
        x: 10.234,
        y: 1.55,
        z: 5.122,
    };
    let v2 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let w: f64 = 7.0;
    println!("Hello, world! {}", &v2 / w);
    println!("Hello, world! {}", &v2 + &v1);

    println!("DOT: {}", v1.normalized().dot(&v2.normalized()));
    println!("CROSS: {}", v1.cross(&v2).normalized());
    let w = 5;
    println!("{}", &v2 / w);
    println!("{}", Vector3::<u32>::zero() * 10.0)
}
