#![feature(partition_point)]
use rand::prelude::*;
use raytracing_one_weekend::camera::Camera;
use raytracing_one_weekend::hit::*;
use raytracing_one_weekend::material::*;
use raytracing_one_weekend::objLoader;
use raytracing_one_weekend::objLoader::*;
use raytracing_one_weekend::ray::Ray;
use raytracing_one_weekend::vectors::Vector3;
use raytracing_one_weekend::vectors::Vector3 as Color;
use std::collections::HashMap;
use std::vec::Vec;
// use raytracing_one_weekend::vectors::Vector3 as P    oint3;
use raytracing_one_weekend::BVH::Bounds;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::{thread, time};
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

pub static RAY_TRI_TESTS: AtomicUsize = AtomicUsize::new(0);
pub static RAY_TRI_ISECT: AtomicUsize = AtomicUsize::new(0);
pub static TRI_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static RAY_COUNT: AtomicUsize = AtomicUsize::new(0);
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
    RAY_COUNT.store(
        RAY_COUNT.load(Ordering::Acquire) + 1 as usize,
        Ordering::Relaxed,
    );
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
                x: 1.05,
                y: 1.1,
                z: 1.2,
            };
            return (1.0 - t) * (botcolor) + (t * topcolor);
        }
    }
}
struct World<'a> {
    objects: HittableList,
    materials: HashMap<&'a str, Arc<dyn Material>>,
}
enum matTypes {
    lambert,
    metal,
    dialectric,
    emissive,
    normal,
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
            matTypes::normal => Arc::new(Normal {}) as Arc<dyn Material>,
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
            matTypes::emissive => Arc::new(Emissive {
                albedo: c,
                emission: rough,
            }),
        };
        self.materials.insert(name, material);
    }
    pub fn addSphere(&mut self, p: (f64, f64, f64), r: f64, mat: &'a str) {
        let m = self.materials.get(mat).unwrap();
        self.objects.add(Box::new(Sphere {
            center: Vector3::from_tuple(p),
            radius: r,
            mat: Arc::clone(m),
            bbox: Bounds::fromSphere(Vector3::from_tuple(p), r),
        }));
    }
    #[rustfmt::skip]
    pub fn addTri(
        &mut self,
        p0: (f64, f64, f64),
        p1: (f64, f64, f64),
        p2: (f64, f64, f64),
        mat: &'a str,
    ) {
        let m = self.materials.get(mat).unwrap();
        let p0 = Vector3::from_tuple(p0);
        let p1 = Vector3::from_tuple(p1);
        let p2 = Vector3::from_tuple(p2);
        let mut bbox = Bounds::new();
        bbox.fitPoints(vec![p0, p1, p2]);
        self.objects.add(Box::new(Tri {
            v0: Vert { P: p0, UV: Vector3::zero(), N: Vector3::zero()},
            v1: Vert { P: p1, UV: Vector3::zero(), N: Vector3::zero()},
            v2: Vert { P: p2, UV: Vector3::zero(), N: Vector3::zero()},
            mat: Arc::clone(m),
            bbox: bbox
        }))
    }
    #[rustfmt::skip]
    pub fn addTriMesh(
        &mut self,
        tris: &Vec<objLoader::TriData>,
        offset: Vector3<f64>,
        mat: &'a str,
    ) {
        let mut mesh = HittableList {
            objects: vec![],
            bbox: Bounds::new(),
        };
        let m = self.materials.get(mat).unwrap();
        for t in tris {
            TRI_COUNT.store(TRI_COUNT.load(Ordering::Acquire) + 1, Ordering::Relaxed);
            // println!("{:?}", t);
            let mut bbox = Bounds::new();
            bbox.fitPoints(vec![t.v0, t.v1, t.v2]);
            mesh.add(Box::new(Tri {
                v0: Vert{ P: t.v0 + offset, UV: t.vt0, N: t.vn0},
                v1: Vert{ P: t.v1 + offset, UV: t.vt1, N: t.vn1},
                v2: Vert{ P: t.v2 + offset, UV: t.vt2, N: t.vn2},
                mat: Arc::clone(m),
                bbox: bbox,
            }));
            mesh.bbox.fitPoints(vec![t.v0,t.v1,t.v2]);
        }
        self.objects.add(Box::new(mesh));
        // self.objects.add(Box::new(Tri {
        //     v0: Vector3::from_tuple(v0),
        //     v1: Vector3::from_tuple(v1),
        //     v2: Vector3::from_tuple(v2),
        //     mat: Arc::clone(m),
        // }))
    }
}
#[rustfmt::skip]
fn makeWorld<'a>() -> World<'a> {
    let mut world = World {
        objects: HittableList {
            objects: vec![],
            bbox: Bounds::infinity(),
        },
        materials: HashMap::new(),
    };
    let OBJ = objLoader::objToTrilist().unwrap();
    world.addMat("green", matTypes::lambert, (0.2, 0.7, 0.3), 0.5, 1.0);
    world.addMat("red", matTypes::lambert, (1.0, 0.1, 0.1), 0.5, 1.0);
    world.addMat("blue", matTypes::lambert, (0.0, 0.1, 0.9), 0.5, 1.0);
    world.addMat("grey", matTypes::lambert, (0.8, 0.8, 0.8), 0.5, 1.0);
    world.addMat("metal", matTypes::metal, (0.2, 0.6, 0.5), 0.4, 1.0);
    world.addMat("glass", matTypes::dialectric, (1.0, 1.0, 1.0), 0.01, 1.5);
    world.addMat("light", matTypes::emissive, (0.9, 0.8, 0.5), 20.0, 0.0);
    world.addMat("normal", matTypes::normal, (0.9, 0.8, 0.5), 20.0, 0.0);
    world.addSphere((0.0, -100.5, -1.0), 100.0, "green");
    world.addSphere((2.0, 0.0, -0.5), 0.6, "red");
    world.addSphere((0.0, 0.0, -3.0), 0.5, "red");
    world.addSphere((-1.0, 0.0, -1.0), 0.5, "red");
    world.addSphere((0.2, -0.0, -1.8), 0.1, "metal");
    world.addSphere((0.6, -0.4, -0.8), 0.1, "metal");
    world.addSphere((0.1, -0.1, -0.3), 0.1, "metal");
    world.addSphere((0.7, -0.0, -0.5), 0.1, "metal");
    world.addSphere((0.3, 0.1, -0.8), 0.1, "metal");
    world.addSphere((0.4, 0.2, -0.8), 0.1, "metal");
    world.addSphere((0.5, 0.3, -0.8), 0.1, "metal");
    world.addSphere((0.6, 0.4, -0.8), 0.1, "metal");
    world.addSphere((0.6, 0.5, -0.8), 0.1, "metal");
    world.addSphere((0.7, 0.6, -0.8), 0.1, "metal");
    world.addSphere((0.8, 0.7, -0.8), 0.1, "metal");
    world.addSphere((0.9, 0.8, -0.8), 0.1, "metal");
    world.addSphere((0.35, 0.1, -0.8), 0.1, "metal");
    world.addSphere((0.45, 0.2, -0.8), 0.1, "metal");
    world.addSphere((0.55, 0.3, -0.8), 0.1, "metal");
    world.addSphere((0.65, 0.4, -0.8), 0.1, "metal");
    world.addSphere((0.65, 0.5, -0.8), 0.1, "metal");
    world.addSphere((0.75, 0.6, -0.8), 0.1, "metal");
    world.addSphere((0.85, 0.7, -0.8), 0.1, "metal");
    world.addSphere((0.95, 0.8, -0.8), 0.1, "metal");
    world.addSphere((0.35 - 0.5, 0.1, -0.8), 0.1, "metal");
    world.addSphere((0.45 - 0.5, 0.2, -0.8), 0.1, "metal");
    world.addSphere((0.55 - 0.5, 0.3, -0.8), 0.1, "metal");
    world.addSphere((0.65 - 0.5, 0.4, -0.8), 0.1, "metal");
    world.addSphere((0.65 - 0.5, 0.5, -0.8), 0.1, "metal");
    world.addSphere((0.75 - 0.5, 0.6, -0.8), 0.1, "metal");
    world.addSphere((0.85 - 0.5, 0.7, -0.8), 0.1, "metal");
    world.addSphere((0.95 - 0.5, 0.8, -0.8), 0.1, "metal");
    // world.addTri(
    //     (1.0, 0.0, -0.6),
    //     (1.0, 1.0, -0.6),
    //     (0.0, 0.0, -1.0),
    //     "green",
    // );
    // world.addTri(
    //     (1.0, -0.5 - 0.2, 0.6),
    //     (1.0, 1.0 - 0.2, -0.6),
    //     (-0.2, -0.9 - 0.2, -1.0),
    //     "blue",
    // );
    world.addTriMesh(
        &OBJ,
        1.5 * Vector3::forward() + (0.2 * Vector3::up()),
        "blue",
    );
    // // let mut triList = vec![((0.0, 0.0, -1.0), (1.0, 1.0, -1.0), (0.0, 1.0, -1.0))]; // TEST FN FOR MESHES
    // triList.push(((1.0, 0.0, 0.6), (1.0, 1.0, 0.6), (0.0, 0.0, -1.2)));
    // world.addTriMesh(triList, "red");
    return world;
}
fn bufferIterator(
    b: &mut u32,
    idx: u64,
    width: usize,
    height: usize,
    sample_count: u32,
    world: &World,
) {
    let cam = Camera::new();
    let max_depth = 20;

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

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

fn main() {
    // Image
    let width = 400 / 8;
    let height = 225 / 8;
    let samplect = 200;

    let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];
    let mut renderbuffer: Vec<u32> = vec![0; width as usize * height as usize];
    let wi = minifb::WindowOptions {
        borderless: true,
        title: false,
        resize: false,
        scale: minifb::Scale::X8,
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

    let world = makeWorld();

    let startTime = Instant::now();
    let mut timed = false;
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
                    &world,
                );
            }
            i += batch;
        }
        if i >= size && !timed {
            println!("TIME      {}", startTime.elapsed().as_millis());
            println!("RAY COUNT {}", RAY_COUNT.load(Ordering::Acquire));
            println!("TRI TESTS {}", RAY_TRI_TESTS.load(Ordering::Acquire));
            println!("TRI ISECT {}", RAY_TRI_ISECT.load(Ordering::Acquire));
            println!("TRI COUNT {}", TRI_COUNT.load(Ordering::Acquire));
            timed = true;
        }
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
