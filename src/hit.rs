use crate::material::Material;
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;

pub struct HitInfo<'a> {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub front_face: bool,
    pub mat: &'a Box<dyn Material>, //SHARED PTR IN TUTORIAL -- MAY NEED TO BE ARC, OR &, OR &MUT
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
        let mut out: Option<HitInfo> = None;
        let mut closest_hit = t_max;
        for o in self.objects.iter() {
            match o.hit(r, t_min, closest_hit) {
                Some(hit) => {
                    closest_hit = hit.t;
                    out = Some(hit);
                }
                _ => {}
            }
        }
        return out;
    }
}
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub mat: Box<dyn Material>, //SHARED PTR IN TUTORIAL -- MAY NEED TO BE ARC, OR &, OR &MUT
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = r.origin - self.center;
        let a = r.dir.sqrmagnitude();
        let half_b = oc.dot(&r.dir);
        let c = oc.sqrmagnitude() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            let hit_p = r.at(temp);
            let outward_normal = (r.at(temp) - self.center) / self.radius;
            let f_face = r.dir.dot(&outward_normal) < 0.0;
            if temp < t_max && temp > t_min {
                return Some(HitInfo {
                    p: hit_p,
                    t: temp,
                    front_face: f_face,
                    normal: match f_face {
                        true => outward_normal,
                        false => &outward_normal * -1.0,
                    },
                    mat: &self.mat,
                });
            };
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return Some(HitInfo {
                    p: hit_p,
                    t: temp,
                    front_face: f_face,
                    normal: match f_face {
                        true => outward_normal,
                        false => &outward_normal * -1.0,
                    },
                    mat: &self.mat,
                });
            }
        }
        return None;
    }
}
