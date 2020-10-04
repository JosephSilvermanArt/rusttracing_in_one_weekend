use crate::vectors::Vector3;
use std::convert::Into;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub invDir: Vector3<f64>,
    pub sign: Vector3<i32>,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return &self.origin + &(&self.dir * t);
    }
    pub fn new(o: Vector3<f64>, d: Vector3<f64>) -> Ray {
        let invdir = 1.0 / &(-1.0 * d);
        Ray {
            origin: o,
            dir: d,
            invDir: invdir,
            sign: Vector3 {
                x: (invdir.x < 0.0) as i32,
                y: (invdir.y < 0.0) as i32,
                z: (invdir.z < 0.0) as i32,
            },
        }
    }
}
