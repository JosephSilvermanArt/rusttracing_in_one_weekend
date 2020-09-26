use crate::hit::{HitInfo, Hittable};
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;

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
