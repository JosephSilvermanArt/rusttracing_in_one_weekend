use crate::ray::Ray;
use crate::vectors::Vector3;
use std::sync::Arc;

pub struct HitInfo {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub front_face: bool,
}

impl HitInfo {
    fn setup_HitInfo(t: f64, p: Vector3<f64>, r: &Ray, outward_normal: &Vector3<f64>) -> HitInfo {
        let f_face = r.dir.dot(outward_normal) < 0.0;
        HitInfo {
            p: p,
            t: t,
            front_face: f_face,
            normal: match f_face {
                true => *outward_normal,
                false => outward_normal * -1.0,
            },
        }
    }
}
pub trait Hittable {
    //Should this return an option, or a bool ith a mutable ref instead, like the book? Is book way better for memory coherency?
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

pub struct Hittable_List<'a> {
    pub objects: &'a mut Vec<Box<dyn Hittable>>,
}
impl<'a> Hittable_List<'a> {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.objects.append(&mut vec![h]);
    }
}

impl<'a> Hittable for Hittable_List<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let mut last_hit = HitInfo {
            t: 0.0,
            p: Vector3::zero(),
            normal: Vector3::zero(),
            front_face: false,
        }; //NOT TO BE RETURNED -- just have to initialize for rust
        let mut hit_anything = false;
        let mut closest_hit = t_max;
        for o in self.objects[..].iter() {
            match o.hit(r, t_min, closest_hit) {
                Some(hit) => {
                    closest_hit = hit.t;
                    last_hit = hit;
                    hit_anything = true;
                }
                _ => {}
            }
        }
        match hit_anything {
            true => Some(last_hit),
            false => None,
        }
    }
}
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = r.origin - self.center;
        let a = r.dir.sqrmagnitude();
        let half_b = oc.dot(&r.dir);
        let c = oc.sqrmagnitude() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitInfo::setup_HitInfo(
                    temp,
                    r.at(temp),
                    r,
                    &(((r.at(temp)) - self.center) / self.radius),
                ));
            };
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitInfo::setup_HitInfo(
                    temp,
                    r.at(temp),
                    r,
                    &(((r.at(temp)) - self.center) / self.radius),
                ));
            };
        }
        return None;
    }
}
