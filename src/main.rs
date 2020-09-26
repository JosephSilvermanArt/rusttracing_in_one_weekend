// use rand::Rng;
use rand::prelude::*;
use raytracing_one_weekend::camera::Camera;
use raytracing_one_weekend::hit::*;
use raytracing_one_weekend::material::*;
use raytracing_one_weekend::ray::Ray;
use raytracing_one_weekend::vectors::Vector3;
use raytracing_one_weekend::vectors::Vector3 as Color;

// use raytracing_one_weekend::vectors::Vector3 as P    oint3;
use std::fs::File;
use std::io::prelude::*;
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
                x: 0.5,
                y: 0.5,
                z: 0.5,
            };
            let topcolor = Color {
                x: 0.6,
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

fn bufferLoop(b: &mut Vec<u32>, width: u32, height: u32, sample_count: u32) {
    let cam = Camera::new();
    let max_depth = 50;

    let mut world = Hittable_List {
        objects: &mut vec![],
    };
    let mat1 = Metal {
        albedo: Color {
            x: 0.6,
            y: 0.6,
            z: 0.6,
        },
        fuzz: 0.1,
    };
    let mat2 = Lambert {
        albedo: Color {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let mat3 = Lambert {
        albedo: Color {
            x: 0.4,
            y: 0.66,
            z: 0.5,
        },
    };
    let mat4 = Dialectric {
        albedo: Color {
            x: 0.0,
            y: 1.0,
            z: 1.0,
        },
        ref_idx: 1.5,
        fuzz: 0.1,
    };
    world.add(Box::new(Sphere {
        center: &Vector3::forward() * 2,
        radius: 0.8,
        mat: Box::new(mat1),
    })); //make_shared<sphere>(point3(0,0,-1), 0.5));
    world.add(Box::new(Sphere {
        center: Vector3::forward() + Vector3::right(),
        radius: 0.5,
        mat: Box::new(mat4),
    }));
    world.add(Box::new(Sphere {
        center: Vector3::forward() + (-1.0 * Vector3::right()),
        radius: 0.5,
        mat: Box::new(mat4),
    })); //make_shared<sphere>(point3(0,0,-1), 0.5));
    world.add(Box::new(Sphere {
        center: Vector3::forward() + (-1.0 * Vector3::right()),
        radius: -0.3,
        mat: Box::new(mat4),
    })); //make_shared<sphere>(point3(0,0,-1), 0.5));
    world.add(Box::new(Sphere {
        center: Vector3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        mat: Box::new(mat2),
    }));

    for j in (0..(height)).rev() {
        let mut rng = thread_rng();
        for i in (0..(width)).rev() {
            let mut pixel_color = Color::zero();
            for k in 0..sample_count {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (width) as f64;
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / (height) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + raycolor(&r, &world, max_depth);
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
    let samplect = 200;

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
        if i == 0 {
            bufferLoop(&mut buffer, width, height, samplect);
        }
        i += 1;
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
