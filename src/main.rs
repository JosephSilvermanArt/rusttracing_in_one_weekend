use raytracing_one_weekend::hit::*;
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

fn raycolor_old(r: &Ray) -> Color<f64> {
    let raydir = r.dir.normalized();
    let s = Sphere {
        center: Vector3::forward(),
        radius: 0.5,
    };
    // let t = hitsphere(
    // &Sphere {
    // center: Vector3::forward(),
    // radius: 0.5,
    // },
    // r,
    // );
    // if t > 0.0 {
    //     let N = (r.at(t) - Vector3::forward()).normalized();
    //     return &Color {
    //         x: N.x + 1.0,
    //         y: N.y + 1.0,
    //         z: N.z + 1.0,
    //     } * 0.5;
    // }
    match s.hit(r, 0.0, f64::INFINITY) {
        Some(h) => {
            let N = h.normal;
            return &Color {
                x: N.x + 1.0,
                y: N.y + 1.0,
                z: N.z + 1.0,
            } * 0.5;
        }
        _ => {
            let t = 0.5 * (raydir.y + 1.0);
            let botcolor = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let topcolor = Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            return (1.0 - t) * (botcolor) + (t * topcolor);
        }
    }
}
fn raycolor<T>(r: &Ray, world: &T) -> Color<f64>
where
    T: Hittable,
{
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(hit) => {
            return ((hit.normal)
                + Color {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                })
            .normalized()
        }
        None => {
            let t = 0.5 * (r.dir.normalized().y + 1.0);
            let botcolor = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let topcolor = Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            return (1.0 - t) * (botcolor) + (t * topcolor);
        }
    }
}

fn bufferLoop(b: &mut Vec<u32>, width: u32, height: u32) {
    // let width = writer.x_size;
    // let height = writer.y_size;
    let mut world = Hittable_List {
        objects: &mut vec![],
    };
    world.add(Box::new(Sphere {
        center: Vector3::forward(),
        radius: 0.5,
    })); //make_shared<sphere>(point3(0,0,-1), 0.5));
    world.add(Box::new(Sphere {
        center: Vector3 {
            x: 0.0,
            y: -105.0,
            z: -100.0,
        },
        radius: 100.0,
    })); //make_shared<sphere>(point3(0,-100.5,-1), 100))

    let viewport_height = 2.0;
    let viewport_width = (width as f64 / height as f64) as f64 * viewport_height;
    let focal_length = 1.0;
    let origin = &Vector3::right() * 1.0; //
    let horizontal = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lens = Vector3 {
        x: 0.0,
        y: 0.0,
        z: focal_length,
    };
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - lens;
    for j in (0..(height)).rev() {
        for i in (0..(width)).rev() {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r = Ray {
                origin: origin,
                dir: lower_left_corner + (u) * horizontal + v * vertical - origin,
            };
            let c = raycolor(&r, &world); //Color { x: r, y: g, z: b };
                                          // let c = raycolor_old(&r); // let c = Color { x: u, y: v, z: 0.5 };
            let idx: usize = (width * height) as usize - 1 - (i as usize + (j * width) as usize);
            b[idx] = from_u8_rgb(
                (255 as f64 * c.x) as u8,
                (255 as f64 * c.y) as u8,
                (255 as f64 * c.z) as u8,
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

    // let writer = PpmWriter::new(width, height, "test").expect("Creating PPMWriter Failed");
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
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600 / 2)));
    // let mut iter = buffer.iter_mut();
    // let size = height * width;
    // let mut i = 0;
    while window.is_open() && !window.is_key_down(Key::C) {
        // if i < size {
        //     for j in 0..size / 4 {
        //         bufferIterator(
        //             &mut buffer[(size - 1 - (i + j)) as usize],
        //             (i + j) as usize,
        //             width,
        //             height,
        //         );
        //     }
        //     i += size / 4;
        // } // BROKEN

        bufferLoop(&mut buffer, width, height);

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
