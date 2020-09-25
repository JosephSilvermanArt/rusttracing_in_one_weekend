use crate::vectors::Vector3;
use std::convert::Into;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return &self.origin + &(&self.dir * t);
    }
}
