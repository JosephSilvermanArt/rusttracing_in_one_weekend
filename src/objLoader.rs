use crate::vectors::Vector3;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

// goal is to return a list of tuples where each tuple is a tuple of tuples of the three tri positions of a face.
// No importing normals, no importing uvs, no connectivity.
//SO:
// Open the file,
// Identify the list of points
// Load the list of points into a vec or array
// Identify the facesd
// Build faces by looking up from points
pub fn objToTrilist() -> Result<Vec<Vec<Vec<f64>>>, std::io::Error> {
    let path = Path::new("./src/blendermonkey.obj"); // TODO, make an argument
    let mut i = 0;
    let mut points = Vec::new(); //make this work
    let mut tris: Vec<Vec<Vec<f64>>> = Vec::new(); //<((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))>; //make this work
                                                   // make buffered reader here //....
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    loop {
        line.clear();
        let result = reader.read_line(&mut line)?;
        if result == 0 {
            break;
        }
        match &line[0..2] {
            "v " => {
                // println!("{}", line);
                let mut iter = line.split_whitespace();
                assert_eq!(Some("v"), iter.next()); // Consume prefix
                let mut position = vec![0.0, 0.0, 0.0];
                position[0] = iter.next().unwrap().parse().unwrap();
                position[1] = iter.next().unwrap().parse().unwrap();
                position[2] = iter.next().unwrap().parse().unwrap();
                points.push(position);
                // println!("{}", points.len());
            }
            "f " => {
                let mut iter = line.split_whitespace();
                assert_eq!(Some("f"), iter.next()); // Consume psdasrefix
                let mut tri: Vec<Vec<f64>> = Vec::new();
                for i in 0..3 {
                    // for each tri
                    let idx: usize = iter.next().unwrap().parse().unwrap();
                    // println!("{} / {}", idx - 1, points.len());
                    let mut cond = false;
                    tri.push(points.get((idx - 1) % points.len()).unwrap().to_vec());
                }
                tris.push(tri);
                // println!("{:?}", tris);
                // position[0] = points
                // .get(inerItter.next().unwrap().parse().unwrap() as usize)
                // .unwrap();
                // position[1] = points
                // .get(inerItter.next().unwrap().parse().unwrap() as usize)
                // .unwrap();
                // position[2] = points
                // .get(inerItter.next().unwrap().parse().unwrap() as usize)
                // .unwrap();
            }
            _ => {}
        }
    }
    // assert_eq!(1, 2);

    return Ok(tris);
}
// E Z
