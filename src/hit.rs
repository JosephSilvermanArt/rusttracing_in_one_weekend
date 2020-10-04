use crate::material::Material;
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;
use crate::BVH::Bounds;
use crate::*;
use std::cmp::{max, min};
use std::ops::Index;
use std::sync::Arc;
pub struct HitInfo<'a> {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub front_face: bool,
    pub mat: &'a Arc<dyn Material>, //SHARED PTR IN TUTORIAL -- MAY NEED TO BE ARC, OR &, OR &MUT
}

pub trait Hittable {
    //Should this return an option, or a bool ith a mutable ref instead, like the book? Is book way better for memory coherency?
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
    fn get_bounds(&self) -> &Bounds;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    pub bbox: Bounds,
}
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.objects.append(&mut vec![h]);
    }
    pub fn generateBVH(&mut self, h: Box<dyn Hittable>) {
        self.objects.append(&mut vec![h]);
    }
}

impl Hittable for HittableList {
    fn get_bounds(&self) -> &Bounds {
        &self.bbox
    }
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
pub struct Vert {
    pub P: Vector3<f64>,
    pub N: Vector3<f64>,
    pub UV: Vector3<f64>,
}
pub struct Tri {
    pub v0: Vert,
    pub v1: Vert,
    pub v2: Vert,
    pub mat: Arc<dyn Material>,
    pub bbox: Bounds,
}
impl Tri {
    fn center(&self) -> Vector3<f64> {
        return (self.v0.P + self.v1.P + self.v2.P) / 3.0;
    }
    fn getBaryCentric(&self, p: Vector3<f64>) -> Vector3<f64> {
        let v0v1 = self.v1.P - self.v0.P;
        let v0v2 = self.v2.P - self.v0.P;
        // no need to normalize
        let N = v0v1.cross(&v0v2);
        let edge0 = self.v1.P - self.v0.P;
        let edge1 = self.v2.P - self.v1.P;
        let edge2 = self.v0.P - self.v2.P;
        // no need to normalize
        // let N = edge0.cross(&edge2);
        let denom = N.dot(&N); // N
        let u = N.dot(&edge1.cross(&(p - self.v1.P))) / denom;
        let v = N.dot(&edge2.cross(&(p - self.v2.P))) / denom;
        let w = 1.0 - u - v;
        return Vector3 { x: u, y: v, z: w };
    }
}

impl Hittable for Tri {
    fn get_bounds(&self) -> &Bounds {
        return &self.bbox;
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let v0v1 = self.v1.P - self.v0.P;
        let v0v2 = self.v2.P - self.v0.P;
        let normal = v0v1.cross(&v0v2).normalized(); // Possibly inverted
        let D = normal.dot(&self.v0.P);
        let t = (-1.0 * normal.dot(&r.origin) + D) / (normal.dot(&r.dir)); // WARNING ABOUT INVERTED HERE IN SCRATCHPIXEL
        if normal.dot(&r.dir).abs() < 0.00000001 {
            // parallel
            return None;
        }
        let pHit = r.at(t);
        //Backface cull -- can we have two sided?
        if t < 0.0 {
            return None;
        }

        // // Starting left test

        //edge 0
        let v0v1 = self.v1.P - self.v0.P;
        let vp0 = pHit - self.v0.P;
        let C = v0v1.cross(&vp0);
        if normal.dot(&C) < 0.0 {
            return None;
        };

        //edge 1
        let v0v1 = self.v2.P - self.v1.P;
        let vp1 = pHit - self.v1.P;
        let C = v0v1.cross(&vp1);
        if normal.dot(&C) < 0.0 {
            return None;
        };
        //edge 2
        let v0v2 = self.v0.P - self.v2.P;
        let vp2 = pHit - self.v2.P;
        let C = v0v2.cross(&vp2);
        if normal.dot(&C) < 0.0 {
            return None;
        };
        let bary = self.getBaryCentric(pHit);

        let temp = t;
        let outward_normal = (bary.x * self.v0.N) + (bary.y * self.v1.N) + (bary.z * self.v2.N);
        let f_face = r.dir.dot(&outward_normal) < 0.0;
        if outward_normal.dot(&r.dir).abs() < 0.00000001 {
            // parallel
            return None;
        }
        if t < t_max && temp > t_min {
            return Some(HitInfo {
                p: pHit,
                t: temp, //think this should be like, distance from origin to t, instead
                normal: match f_face {
                    true => outward_normal,
                    false => &outward_normal * -1.0,
                },
                front_face: !f_face,
                mat: &self.mat,
            });
        } else {
            return None;
        }

        // let projVec = r.dir.cross(&v0v2);
        // let determinant = edge0.dot(&projVec);

        // let projected = r.at((r.origin - self.projectToTri(r.dir)).sqrmagnitude());
        // let oc = projected - (r.origin * &r.dir);
        // let a = r.dir.sqrmagnitude();
        // let half_b = oc.dot(&r.dir);
        // // assert_eq!(r.at((r.origin - projected).sqrmagnitude()), projected);
        // match &self.triContains(projected) {
        //     true => {
        //         let discriminant = (half_b * half_b) * (a * oc.sqrmagnitude());
        // // So the idea is gonna be....
        //project any point along the ray (let's say origin) onto the tris plane
        // Check if that hit point is the same direciton as ray
        // Check if that hit point is intersecting tri
        // Check if that hit point is within range
        // If so, hit!! returning the tris normal, and any vert info thru barycentric lookup
    }
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub mat: Arc<dyn Material>, //SHARED PTR IN TUTORIAL -- MAY NEED TO BE ARC, OR &, OR &MUT
    pub bbox: Bounds,
}
impl Hittable for Sphere {
    fn get_bounds(&self) -> &Bounds {
        &self.bbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }
        let oc = r.origin - self.center;
        let a = r.dir.sqrmagnitude();
        let half_b = oc.dot(&r.dir);
        let c = oc.sqrmagnitude() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let hit_p = r.at(temp);
                let outward_normal = (r.at(temp) - self.center) / self.radius;
                let f_face = r.dir.dot(&outward_normal) < 0.0;
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
                let hit_p = r.at(temp);
                let outward_normal = (r.at(temp) - self.center) / self.radius;
                let f_face = r.dir.dot(&outward_normal) < 0.0;
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
