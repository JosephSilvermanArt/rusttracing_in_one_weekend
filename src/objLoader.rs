use crate::vectors::Vector3;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

pub fn objToTrilistold() -> Result<Vec<Vec<Vec<f64>>>, Box<dyn std::error::Error>> {
    let path = Path::new("./src/blendermonkey.obj");
    let mut i = 0;
    let mut points = Vec::new();
    let mut tris: Vec<Vec<Vec<f64>>> = Vec::new(); //<((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))>; //make this work
    let f = File::open(path)?;
    let mut reader = BufReader::new(&f);
    let mut i = 0;
    let mut line = String::new();
    loop {
        line.clear();
        let result = reader.read_line(&mut line)?;
        if result == 0 {
            break;
        }
        match &line[0..2] {
            "v " => {
                let mut iter = line.split_whitespace();
                assert_eq!(Some("v"), iter.next()); // Consume prefix
                let mut position = vec![0.0, 0.0, 0.0];
                position[0] = iter.next().unwrap().parse()?;
                position[1] = iter.next().unwrap().parse()?;
                position[2] = iter.next().unwrap().parse()?;
                points.push(position);
            }
            "f " => {
                let mut iter = line.split_whitespace();
                assert_eq!(Some("f"), iter.next()); // Consume prefix
                let mut tri: Vec<Vec<f64>> = Vec::new();
                for i in 0..3 {
                    // for each tri
                    let idx: usize = iter.next().unwrap().parse()?;
                    let mut cond = false;
                    tri.push(points.get((idx - 1) % points.len()).unwrap().to_vec());
                }
                tris.push(tri);
            }
            _ => {}
        }
    }
    return Ok(tris);
}

pub struct TriData {
    pub v1:Vector3<f64>,
    pub v0:Vector3<f64>,
    pub v2:Vector3<f64>,
    pub vt0:Vector3<f64>,
    pub vt1:Vector3<f64>,
    pub vt2:Vector3<f64>,
    pub vn0:Vector3<f64>,
    pub vn1:Vector3<f64>,
    pub vn2:Vector3<f64>,
}
//TODO -- struct for imported faces. Return vec of that. Make fn to create a mesh from them. include normals.
pub fn objToTrilist() -> Result<Vec<TriData>, Box<dyn std::error::Error>> {
    let path = Path::new("./src/blendermonkey_attributes.obj"); // TODO, make an argument
    let mut i = 0;
    let mut points = Vec::new(); //make this work
    let mut UVs = Vec::new(); //make this work
    let mut normals = Vec::new(); //make this work
    let mut tris: Vec<TriData> = Vec::new(); //<((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))>; //make this work
                                             // make lineed reader here //....
    let f = File::open(path)?;
    let reader = BufReader::new(&f);
    let mut i = 0;
    let mut hasUVs = false;
    let mut hasNormals = false;
    for line in reader.lines() {
        let text = &line?[..];
        i += 1;
        match &text.split_whitespace().next() {
            Some("v") => {
                let mut iter = text.trim().split_whitespace();
                assert_eq!(Some("v"), iter.next()); // Consume prefix
                let mut position = vec![0.0, 0.0, 0.0];
                position[0] = iter.next().unwrap().parse()?;
                position[1] = iter.next().unwrap().parse()?;
                position[2] = iter.next().unwrap().parse()?;
                points.push(position);
            }
            Some("vn") => {
                if !hasNormals {
                    hasNormals = true
                };
                let mut iter = text.trim().split_whitespace();
                assert_eq!(Some("vn"), iter.next()); // Consume prefix
                let mut position = vec![0.0, 0.0, 0.0];
                position[0] = iter.next().unwrap().parse()?;
                position[1] = iter.next().unwrap().parse()?;
                position[2] = iter.next().unwrap().parse()?;
                normals.push(position);
            }
            Some("vt") => {
                if !hasUVs {
                    hasUVs = true
                };
                let mut iter = text.trim().split_whitespace();
                assert_eq!(Some("vt"), iter.next()); // Consume prefix
                let mut position = vec![0.0, 0.0, 0.0];
                position[0] = iter.next().unwrap().parse()?;
                position[1] = iter.next().unwrap().parse()?;
                UVs.push(position);
            }
            _ => {}
        }
    }
    let mut reader = BufReader::new(&f);
    reader.seek(std::io::SeekFrom::Start(0));
    for line in reader.lines() {
        let text = &line?[..];
        match &text[0..2] {
            "f " => {
                let mut triP: Vec<Vec<f64>> = Vec::new();
                let mut triN: Vec<Vec<f64>> = Vec::new();
                let mut triVT: Vec<Vec<f64>> = Vec::new();
                let mut faceIter = text.trim().split_whitespace();
                assert_eq!(Some("f"), faceIter.next()); // Consume prefix
                for i in 0..3 {
                    // for each vert
                    match hasUVs || hasNormals {
                        //Incomplete, there are probably more params that can cause /s
                        true => {
                            let mut componentIter = faceIter.next().unwrap().trim().split("/");
                            let idx: usize = componentIter.next().unwrap().parse()?;
                            triP.push(points.get((idx - 1) % points.len()).unwrap().to_vec());
                            
                            let idx: usize = componentIter.next().unwrap().parse()?;
                            if hasUVs {
                                triVT.push(UVs.get((idx - 1) % UVs.len()).unwrap().to_vec());
                            }
                            if hasNormals {
                                let idx: usize = componentIter.next().unwrap().parse()?;
                                triN.push(normals.get((idx - 1) % normals.len()).unwrap().to_vec());
                            }
                        }
                        false => {
                            let idx: usize = faceIter.next().unwrap().parse()?;
                            triP.push(points.get((idx - 1) % points.len()).unwrap().to_vec());
                        }
                    }
                }
                #[rustfmt::skip] 
                tris.push(TriData {
                    v0: Vector3::from_vector(triP[0].to_vec()),
                    v1: Vector3::from_vector(triP[1].to_vec()),
                    v2: Vector3::from_vector(triP[2].to_vec()),
                    vt0: if hasUVs {Vector3::from_vector(triVT[0].to_vec())} else {Vector3::zero()},
                    vt1: if hasUVs {Vector3::from_vector(triVT[1].to_vec())} else {Vector3::zero()},
                    vt2: if hasUVs {Vector3::from_vector(triVT[2].to_vec())} else {Vector3::zero()},
                    vn0: if hasNormals {Vector3::from_vector(triN[0].to_vec())} else {Vector3::zero()},
                    vn1: if hasNormals {Vector3::from_vector(triN[1].to_vec())} else {Vector3::zero()},
                    vn2: if hasNormals { Vector3::from_vector(triN[2].to_vec())} else {Vector3::zero()},
                });
            }
            _ => {}
        }
    }
    return Ok(tris);
}
