// use rand::Rng;
use rand::prelude::*;
use raytracing_one_weekend::camera::Camera;
use raytracing_one_weekend::hit::*;
use raytracing_one_weekend::material::*;
use raytracing_one_weekend::ray::Ray;
use raytracing_one_weekend::vectors::Vector3;
use raytracing_one_weekend::vectors::Vector3 as Color;
use std::collections::HashMap;
use std::vec::Vec;
// use raytracing_one_weekend::vectors::Vector3 as P    oint3;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::{thread, time};
extern crate minifb;

use minifb::{Key, Window, WindowOptions};
// fn buildppm() -> std::io::Result<()> {
//     let w = 256;
//     let h = 256;
//     let mut file = File::create("test.ppm")?;
//     file.write(format!("P3 \n{} {} \n255 \n", w, h).as_bytes())?;
//     for i in (0..(h)).rev() {
//         if (i % (h / 8)) == 0 {
//             println!("{}/{} lines complete", 256 - i, h);
//         };
//         for j in (0..(w)).rev() {
//             let r = i as f64 / (h - 1) as f64;
//             let g = j as f64 / (w - 1) as f64;
//             let b = 0.5;
//             file.write(
//                 format!(
//                     "{} {} {} ",
//                     (r * 255.999) as i64,
//                     (g * 255.999) as i64,
//                     (b * 255.999) as i64
//                 )
//                 .as_bytes(),
//             )?;
//         }
//         file.write(b"\n");
//     }
//     Ok(())
// }
// #[derive(Debug, Clone, PartialEq)]
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}
fn hitsphere(s: &Sphere, r: &Ray) -> f64 {
    let oc = r.origin - s.center;
    let a = r.dir.sqrmagnitude();
    let half_b = oc.dot(&r.dir);
    let c = oc.sqrmagnitude() - (s.radius * s.radius);
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return -half_b - discriminant.sqrt() / a;
    }
}

fn raycolor<T>(r: &Ray, world: &T, depth: u32) -> Color<f64>
where
    T: Hittable,
{
    if depth <= 0 {
        return Vector3::zero();
    }
    match world.hit(r, 0.001, f64::INFINITY) {
        Some(hit) => match hit.mat.scatter(r, &hit) {
            Some(result) => return result.attenuation * &raycolor(&result.ray, world, depth - 1),
            None => return Color::zero(),
        },
        None => {
            let t = 0.5 * (r.dir.normalized().y + 1.0);
            let botcolor = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let topcolor = Color {
                x: 0.0,
                y: 0.6,
                z: 1.0,
            };
            return (1.0 - t) * (botcolor) + (t * topcolor);
        }
    }
}
// fn makeWorld() -> Hittable_List<'a> {
// let mat_ground = material.
// let mut world = Hittable_List {
// objects: &mut vec![],
// };
struct World<'a> {
    objects: Hittable_List<'a>,
    materials: HashMap<&'a str, Arc<dyn Material>>,
}
enum matTypes {
    lambert,
    metal,
    dialectric,
}
impl<'a> World<'a> {
    pub fn addMat(
        &mut self,
        name: &'a str,
        m: matTypes,
        col: (f64, f64, f64),
        rough: f64,
        ior: f64,
    ) {
        let c = Color {
            x: col.0,
            y: col.1,
            z: col.2,
        };
        let material = match m {
            matTypes::lambert => Arc::new(Lambert { albedo: c }) as Arc<dyn Material>,
            matTypes::metal => Arc::new(Metal {
                albedo: c,
                fuzz: rough,
            }) as Arc<dyn Material>,
            matTypes::dialectric => Arc::new(Dialectric {
                albedo: c,
                fuzz: rough,
                ref_idx: ior,
            }) as Arc<dyn Material>,
        };
        self.materials.insert(name, material);
    }
    pub fn addSphere(&mut self, p: (f64, f64, f64), r: f64, mat: &'a str) {
        let pos = Vector3 {
            x: p.0,
            y: p.1,
            z: p.2,
        };
        let m = self.materials.get(mat).unwrap();
        self.objects.add(Box::new(Sphere {
            center: pos,
            radius: r,
            mat: Arc::clone(m),
        }));
    }
}
fn bufferIterator(b: &mut u32, idx: u64, width: usize, height: usize, sample_count: u32) {
    let cam = Camera::new();
    let max_depth = 50;

    let mut world = World {
        objects: Hittable_List {
            objects: &mut vec![],
        },
        materials: HashMap::new(),
    };
    world.addMat("green", matTypes::lambert, (0.2, 0.7, 0.3), 0.5, 1.0);
    world.addMat("grey", matTypes::lambert, (0.8, 0.8, 0.8), 0.5, 1.0);
    world.addMat("metal", matTypes::metal, (0.5, 0.5, 0.5), 0.5, 1.0);
    world.addMat("glass", matTypes::dialectric, (1.0, 1.0, 1.0), 0.02, 1.5);
    world.addSphere((0.0, -105.0, -1.0), 100.0, "green");
    world.addSphere((1.0, 0.0, -1.0), 0.5, "green");
    world.addSphere((0.0, 0.0, -1.0), 0.5, "metal");
    world.addSphere((-1.0, 0.0, -1.0), 0.5, "glass");

    let i = idx % width as u64;
    let j = idx / (width) as u64;
    let mut rng = thread_rng();
    let mut pixel_color = Color::zero();
    for k in 0..sample_count {
        let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (width) as f64;
        let v = (j as f64 + rng.gen_range(0.0, 1.0)) / (height) as f64;
        let r = cam.get_ray(u, v);
        pixel_color = pixel_color + raycolor(&r, &world.objects, max_depth);
    }
    let c = &pixel_color * (1.0 / sample_count as f64); //divide color by samplect
    let idx: usize = idx as usize;
    let color = from_u8_rgb(
        (255 as f64 * clamp(c.x.sqrt(), 0.0, 1.0)) as u8,
        (255 as f64 * clamp(c.y.sqrt(), 0.0, 1.0)) as u8,
        (255 as f64 * clamp(c.z.sqrt(), 0.0, 1.0)) as u8,
    );
    *b = from_u8_rgb(
        (255 as f64 * clamp(c.x.sqrt(), 0.0, 1.0)) as u8,
        (255 as f64 * clamp(c.y.sqrt(), 0.0, 1.0)) as u8,
        (255 as f64 * clamp(c.z.sqrt(), 0.0, 1.0)) as u8,
    );
}
fn bufferLoop(b: &mut Vec<u32>, width: u32, height: u32, sample_count: u32) {
    let cam = Camera::new();
    let max_depth = 50;

    let mut world = World {
        objects: Hittable_List {
            objects: &mut vec![],
        },
        materials: HashMap::new(),
    };
    world.addMat("green", matTypes::lambert, (0.2, 0.7, 0.3), 0.5, 1.0);
    world.addMat("grey", matTypes::lambert, (0.8, 0.8, 0.8), 0.5, 1.0);
    world.addMat("metal", matTypes::metal, (0.5, 0.5, 0.5), 0.5, 1.0);
    world.addMat("glass", matTypes::dialectric, (1.0, 1.0, 1.0), 0.02, 1.5);
    world.addSphere((0.0, -105.0, -1.0), 100.0, "green");
    world.addSphere((1.0, 0.0, -1.0), 0.5, "green");
    world.addSphere((0.0, 0.0, -1.0), 0.5, "metal");
    world.addSphere((-1.0, 0.0, -1.0), 0.5, "glass");

    for j in (0..(height)).rev() {
        let mut rng = thread_rng();
        for i in (0..(width)).rev() {
            let mut pixel_color = Color::zero();
            for k in 0..sample_count {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (width) as f64;
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / (height) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + raycolor(&r, &world.objects, max_depth);
            }
            let c = &pixel_color * (1.0 / sample_count as f64); //divide color by samplect
            let idx: usize = (width * height) as usize - 1 - (i as usize + (j * width) as usize);
            b[idx] = from_u8_rgb(
                (255 as f64 * clamp(c.x.sqrt(), 0.0, 1.0)) as u8,
                (255 as f64 * clamp(c.y.sqrt(), 0.0, 1.0)) as u8,
                (255 as f64 * clamp(c.z.sqrt(), 0.0, 1.0)) as u8,
            );
        }
    }
}
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn main() {
    // Image
    let width = 400;
    let height = 225;
    let samplect = 100;

    let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];
    let mut renderbuffer: Vec<u32> = vec![0; width as usize * height as usize];
    let wi = minifb::WindowOptions {
        borderless: true,
        title: false,
        resize: false,
        scale: minifb::Scale::X2,
        topmost: false,
        transparency: false,
        scale_mode: minifb::ScaleMode::Stretch,
    };
    let mut window =
        Window::new("test - esc to exit", width as usize, height as usize, wi).unwrap();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut iter = buffer.iter_mut();
    let size = height * width;
    let mut i = 0;
    while window.is_open() && !window.is_key_down(Key::C) {
        if i < size {
            let batch = size / (2 ^ 16);
            for j in i..(i + batch) {
                if j > size - 1 {
                    break;
                }
                bufferIterator(
                    &mut buffer[(size - 1 - j)],
                    (j) as u64,
                    width as usize,
                    height as usize,
                    samplect,
                );
            }
            i += batch;
        }
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
