use crate::hit::{HitInfo, Hittable, Tri};
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;
use rand::prelude::*;
use std::cmp;

pub fn schlick(cos: f64, index: f64) -> f64 {
    let mut r0 = (1.0 - index) / (1.0 + index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result>; // DOES THIS NEED TO RETURN SCATTERED INSTEAD?
}
pub struct scatter_result {
    pub attenuation: Color<f64>,
    pub ray: Ray,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambert {
    pub albedo: Color<f64>,
}

impl Material for Lambert {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result> {
        let scatter_direction = hit.normal + Vector3::<f64>::random_unit_vector();
        let result_scattered = Ray {
            origin: hit.p,
            dir: scatter_direction,
        };
        let result_attennuation = self.albedo; // Need to manually copy/clone?
        return Some(scatter_result {
            attenuation: result_attennuation,
            ray: result_scattered,
        });
    }
}
pub struct Emissive {
    pub albedo: Color<f64>,
    pub emission: f64,
}
impl Material for Emissive {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result> {
        let scatter_direction = Vector3::<f64>::random_unit_vector();
        let result_scattered = Ray {
            origin: hit.p,
            dir: scatter_direction,
        };
        let result_attennuation = self.emission * self.albedo; // Need to manually copy/clone?
        return Some(scatter_result {
            attenuation: result_attennuation,
            ray: result_scattered,
        });
    }
}
pub struct Normal {}

impl Material for Normal {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result> {
        let scatter_direction = hit.normal + Vector3::<f64>::random_unit_vector();
        let result_scattered = Ray {
            origin: hit.p,
            dir: scatter_direction,
        };
        let result_attennuation = hit.normal; // Need to manually copy/clone?
        return Some(scatter_result {
            attenuation: result_attennuation,
            ray: result_scattered,
        });
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
    pub albedo: Color<f64>,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result> {
        let scatter_direction = Vector3::<f64>::reflect(r.dir.normalized(), hit.normal);
        let result_scattered = Ray {
            origin: hit.p,
            dir: scatter_direction + (&Vector3::<f64>::random_in_unitsphere() * self.fuzz),
        };
        let result_attennuation = self.albedo; // Need to manually copy/clone?
        match scatter_direction.dot(&hit.normal) > 0.0 {
            true => {
                return Some(scatter_result {
                    attenuation: result_attennuation,
                    ray: result_scattered,
                })
            }
            false => return None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dialectric {
    pub albedo: Color<f64>,
    pub ref_idx: f64,
    pub fuzz: f64,
}
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}
//Added some stupid  fuzz and color stuff here -- remember to remove later on if it causes trouble
impl Material for Dialectric {
    fn scatter(&self, r: &Ray, hit: &HitInfo) -> Option<scatter_result> {
        let result_attennuation = Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }; //self.albedo; // Need to manually copy/clone?
        let index = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_dir = r.dir.normalized();
        let cos_theta = (&unit_dir * -1.0).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        match index * sin_theta > 1.0 {
            true => {
                let result_attennuation = self.albedo;
                let scatter_direction = Vector3::<f64>::reflect(unit_dir, hit.normal);
                let result_ray = Ray {
                    origin: hit.p,
                    dir: scatter_direction + (&Vector3::<f64>::random_in_unitsphere() * self.fuzz),
                };
                return Some(scatter_result {
                    attenuation: result_attennuation,
                    ray: result_ray,
                });
            }
            false => {
                let mut rng = thread_rng();

                match schlick(cos_theta, index) > rng.gen_range(0.0, 1.0) {
                    true => {
                        let result_attennuation = self.albedo;
                        let scatter_direction = Vector3::<f64>::reflect(unit_dir, hit.normal);
                        let result_ray = Ray {
                            origin: hit.p,
                            dir: scatter_direction
                                + (&Vector3::<f64>::random_in_unitsphere() * self.fuzz),
                        };
                        return Some(scatter_result {
                            attenuation: result_attennuation,
                            ray: result_ray,
                        });
                    }

                    false => {
                        let t = clamp(0.7 + self.fuzz, 0.0, 1.0);
                        let result_attennuation =
                            (&result_attennuation * t) + (&self.albedo * (1.0 - t));
                        let scatter_direction =
                            Vector3::<f64>::refract(unit_dir, hit.normal, index);
                        let result_ray = Ray {
                            origin: hit.p,
                            dir: scatter_direction
                                - (&Vector3::<f64>::random_in_unitsphere() * self.fuzz),
                        };
                        return Some(scatter_result {
                            attenuation: result_attennuation,
                            ray: result_ray,
                        });
                    }
                }
            }
        }

        // match scatter_direction.dot(&hit.normal) > 0.0 {
        //     true => {
        //         return Some(scatter_result {
        //             attenuation: result_attennuation,
        //             ray: result_scattered,
        //         })
        //     }
        //     false => return None,
        // }
    }
}
