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
// fn projectToTri(&self, p: Vector3<f64>) -> Vector3<f64> {
//     let origin = self.center();
//     let tri_normal = (self.v1 - self.v0).cross(&(self.p2 - self.v0)).normalized();
//     let oc = p - origin - v2;
//     let cos = oc.dot(&tri_normal);
//     let sub = &tri_normal * cos;
//     return p - sub;
// }
// fn getBaryCentric(&self, p: Vector3<f64>) -> Vector3<f64> // NOTE -- NOT SURE COMPARISONS WORK
// {
// if &self.v0 == &self.v1 && &self.v1 == &self.p2 {
// zero area triav2lev2            match &self.v0 == &p {
// true => {
// return Vector3 {
// x: 1.0,
// y: 0.0,
// z: 0.0,
// }
// }
// false => {
// return Vector3 {
// miss
// x: -100.0,
// y: -100.0,
// z: -100.0,
// };
// }
// }
// }
// let v0 = self.v1 - self.v0;
// let v1 = self.v2 - self.v0;
// let v2 = p - self.v0;
// let den = v0.dot(&v1);
// let invden = 1.0 / den;
// let v = ((v2.x * v1.y) - v1.x * v2.y) * invden;
// let w = ((v0.x * v2.y) - v2.x * v0.y) * invden;
// let u = 1.0 - v - w;
// Vector3 { x: u, y: v, z: w }
// }
// fn triContains(&self, p: Vector3<f64>) -> bool {
// let pBary = self.getBaryCentric(p);
// return 0.0 <= pBary.x
// && pBary.x <= 1.0
// && 0.0 <= pBary.y
// && pBary.y <= 1.0
// && 0.0 <= pBary.z
// && pBary.z <= 1.0;
// }
// }
//SIMPLE SPHERE
// bool hit_sphere(const point3& center, double radius, const ray& r) {
//     vec3 oc = r.origin() - center; //  ray between 0.0 and (target center - origin), pointing opposite way
//     auto a = r.direction.sqrmagnitude; //  sqr size of ray (sqr is cheaper) -- this should  be 1.0 right? its a unit vector??
//     auto  half_b = dot(oc, r.direction()); // so dot direction * oc is -1 if ray is going right at, 1 if its going opp way
//     auto c = oc.sqrmagnitude - radius*radius; // so this is, oc.magnitude - radius for all intents and purposes. Distance to outside of sphere
//     auto discriminant = half_b*half_b - a*c; right side -- size of ray, times distance to surface, times 4.
// //if (discriminant < 0) {
//     return -1.0;
//    } else {
//        return (-half_b - sqrt(discriminant) ) / a;
//    }
//        return (discriminant > 0);
// }
impl Hittable for Tri {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let normal = -1.0 * edge1.cross(&edge2).normalized(); // Possibly inverted
        let D = normal.dot(&self.v0);
        let t = (normal.dot(&r.origin) + D) / (normal.dot(&r.dir)); // WARNING ABOUT INVERTED HERE IN SCRATCHPIXEL
        if normal.dot(&r.dir).abs() < 0.00000001 {
            // parallel
            return None;
        }
        let pHit = r.at(t);
        //Backface cull -- can we have two sided?
        // if t < 0.0 {
        // return None;
        // }

        // Starting left test

        //edge 0
        let edge0 = self.v1 - self.v0;
        let vp0 = pHit - self.v0;
        let C = edge0.cross(&vp0);
        if normal.dot(&C) > 0.0 {
            return None;
        };

        //edge 1
        let edge0 = self.v2 - self.v1;
        let vp1 = pHit - self.v1;
        let C = edge0.cross(&vp1);
        if normal.dot(&C) > 0.0 {
            return None;
        };
        //edge 2
        let edge0 = self.v0 - self.v2;
        let vp2 = pHit - self.v2;
        let C = edge0.cross(&vp2);
        if normal.dot(&C) > 0.0 {
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
