use crate::material::Material;
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;
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
}

pub struct Hittable_List {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl Hittable_List {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, h: Box<dyn Hittable>) {
        self.objects.append(&mut vec![h]);
    }
}

impl Hittable for Hittable_List {
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
pub struct Tri {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub mat: Arc<dyn Material>,
}
impl Tri {
    fn center(&self) -> Vector3<f64> {
        return (self.v0 + self.v1 + self.v2) / 3.0;
    }
}
impl Hittable for Tri {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let normal = edge1.cross(&edge2).normalized(); // Possibly inverted
        let D = normal.dot(&self.v0);
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

        // Starting left test

        //edge 0
        let edge0 = self.v1 - self.v0;
        let vp0 = pHit - self.v0;
        let C = edge0.cross(&vp0);
        if normal.dot(&C) < 0.0 {
            return None;
        };

        //edge 1
        let edge1 = self.v2 - self.v1;
        let vp1 = pHit - self.v1;
        let C = edge1.cross(&vp1);
        if normal.dot(&C) < 0.0 {
            return None;
        };
        //edge 2
        let edge2 = self.v0 - self.v2;
        let vp2 = pHit - self.v2;
        let C = edge2.cross(&vp2);
        if normal.dot(&C) < 0.0 {
            return None;
        };

        let temp = t;
        if t < t_max && temp > t_min {
            return Some(HitInfo {
                p: pHit,
                t: temp, //think this should be like, distance from origin to t, instead
                normal: normal,
                front_face: true,
                mat: &self.mat,
            });
        } else {
            return None;
        }

        // let projVec = r.dir.cross(&edge2);
        // let determinant = edge1.dot(&projVec);

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
