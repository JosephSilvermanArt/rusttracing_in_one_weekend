use crate::hit::HittableList;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vectors::Vector3;
use crate::vectors::Vector3 as Color;
use std::cmp::Ordering;
use std::cmp::{max, min};
use std::ops::Index;
pub enum axis {
    x,
    y,
    z,
}
#[derive(Copy, Clone)]
pub struct Bounds {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
    center: Vector3<f64>,
}
impl std::ops::Index<i32> for Bounds {
    type Output = Vector3<f64>;

    fn index(&self, idx: i32) -> &Self::Output {
        match idx {
            1 => &self.max,
            0 => &self.min,
            _ => &self.min,
        }
    }
}
impl Bounds {
    pub fn infinity() -> Bounds {
        Bounds {
            min: std::f64::NEG_INFINITY * Vector3::one(),
            max: std::f64::INFINITY * Vector3::one(),
            center: Vector3::zero(),
        }
    }
    pub fn new() -> Bounds {
        Bounds {
            min: std::f64::INFINITY * Vector3::one(),
            max: std::f64::NEG_INFINITY * Vector3::one(),
            center: Vector3::zero(),
        }
    }
    pub fn fromSphere(p: Vector3<f64>, r: f64) -> Bounds {
        Bounds {
            min: p + (-r * Vector3::one()),
            max: p + (r * Vector3::one()),
            center: p,
        }
    }
    //fix this later
    pub fn fitPoints(&mut self, points: Vec<Vector3<f64>>) {
        for point in points.iter() {
            self.max.x = self.max.x.max(point.x);
            self.max.y = self.max.y.max(point.y);
            self.max.z = self.max.z.max(point.z);
            self.min.x = self.min.x.min(point.x);
            self.min.y = self.min.y.min(point.y);
            self.min.z = self.min.z.min(point.z);
        }
        self.center = Vector3 {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
            z: self.max.z - self.min.z,
        };
    }
    pub fn from_hittables(objects: &Vec<Box<dyn Hittable>>) -> Bounds {
        let mut b = Bounds::new();
        for o in objects.iter() {
            b.max.x = b.max.x.max(o.get_bounds().max.x);
            b.max.y = b.max.y.max(o.get_bounds().max.y);
            b.max.z = b.max.z.max(o.get_bounds().max.z);
            b.min.x = b.min.x.min(o.get_bounds().min.x);
            b.min.y = b.min.y.min(o.get_bounds().min.y);
            b.min.z = b.min.z.min(o.get_bounds().min.z);
        }
        b.center = Vector3 {
            x: b.max.x - b.min.x,
            y: b.max.y - b.min.y,
            z: b.max.z - b.min.z,
        };
        return b;
    }
    pub fn getLongestAxis(&self) -> axis {
        let mut longest = 0.0;
        let result;
        let dist = |a: f64, b: f64| (a - b).abs();
        if dist(self.max.x, self.min.x) > dist(self.max.y, self.min.y) {
            longest = dist(self.max.x, self.min.x);
            result = axis::x;
        } else {
            longest = dist(self.max.y, self.min.y);
            result = axis::y;
        }
        return if dist(self.max.z, self.min.z) > longest {
            axis::z
        } else {
            result
        };
    }
    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        let mut tmin: f64 = (self[r.sign.x].x - r.origin.x) * r.invDir.x;
        let mut tmax: f64 = (self[1 - r.sign.x].x - r.origin.x) * r.invDir.x;
        let mut tymin: f64 = (self[r.sign.y].y - r.origin.y) * r.invDir.y;
        let mut tymax: f64 = (self[1 - r.sign.y].y - r.origin.y) * r.invDir.y;

        if ((tmin > tymax) || (tymin > tmax)) {
            return false;
        }
        if (tymin > tmin) {
            tmin = tymin;
        }
        if (tymax < tmax) {
            tmax = tymax;
        };
        let mut tzmin = (self[r.sign.z].z - r.origin.z) * r.invDir.z;
        let mut tzmax = (self[1 - r.sign.z].z - r.origin.z) * r.invDir.z;

        if ((tmin > tzmax) || (tzmin > tmax)) {
            return false;
        }
        if (tzmin > tmin) {
            tmin = tzmin;
        }
        if (tzmax < tmax) {
            tmax = tzmax;
        }

        return true;
    }
}
#[derive(Clone)]
struct bvhNode<'a> {
    hittable_list: &'a HittableList,
    indices: Vec<usize>,
    pub left: Option<Box<bvhNode<'a>>>, //BVH Node if branch, HittableLists if leaf,
    pub right: Option<Box<bvhNode<'a>>>, //BVH Node if branch, HittableLists if leaf,
    pub bbox: Bounds,
}

impl<'a> Hittable for bvhNode<'a> {
    fn get_bounds(&self) -> &Bounds {
        &self.bbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let slice = &self.indices;
        // Leaf
        if slice.len() < 3 {
            let mut out: Option<HitInfo> = None;
            let mut closest_hit = t_max;
            for i in self.indices.iter() {
                match self.hittable_list.objects[*i].hit(r, t_min, closest_hit) {
                    Some(hit) => {
                        closest_hit = hit.t;
                        out = Some(hit);
                    }
                    _ => {}
                }
            }
        }
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }
        if self.left.as_ref().unwrap().bbox.hit(r, t_min, t_max) {
            return self.left.as_ref().unwrap().hit(r, t_min, t_max);
        } else {
            return self.right.as_ref().unwrap().hit(r, t_min, t_max);
        }
    }
}

impl<'a> bvhNode<'a> {
    //Kinda confused on how to think about the hittable list. Need to be able to mutate it for sort --
    //Should it consume the hittable list, and then pass a mutable ref down theu recursive fn?
    //Or just take mtable ref and worry abt the memory elsewhere?

    pub fn create_from_hlist(list: &HittableList) -> Option<bvhNode> {
        let i = [0..list.objects.len()].iter().collect();
        return bvhNode::from_HittableList(list, i);
    }

    //TODO: takes in integers from 0..len as indices the first call -- wrap this?
    fn from_HittableList(list: &HittableList, indices: Vec<usize>) -> Option<bvhNode> {
        // let objects = list.objects;
        let p;
        let mut slice = (indices[..].to_vec());
        let mut bbox = Bounds::new(); //= Bounds::from_hittables(&list.objects, s, e);
        for i in slice.iter() {
            let o = &list.objects[*i];
            bbox.fitPoints(vec![o.get_bounds().min, o.get_bounds().max]);
        }
        let longestAxis = bbox.getLongestAxis();
        //base case
        if slice.len() > 3 {
            return Some(bvhNode {
                hittable_list: list,
                indices: slice.to_owned(),
                bbox: bbox,
                left: None,  // TODO, CALL THIS FN
                right: None, // TODO, CALL THIS FN
            });
        }

        #[rustfmt::skip]
        slice.sort_by(|a, b| match longestAxis {
            axis::x => list.objects[*a].get_bounds().center.x.partial_cmp(&list.objects[*b].get_bounds().center.x).unwrap(),
            axis::y => list.objects[*a].get_bounds().center.y.partial_cmp(&list.objects[*b].get_bounds().center.y).unwrap(),
            axis::z => list.objects[*a].get_bounds().center.z.partial_cmp(&list.objects[*b].get_bounds().center.z).unwrap(),
        });

        p = slice.partition_point(|a| match longestAxis {
            axis::x => list.objects[*a].get_bounds().center.x > bbox.center.x,
            axis::y => list.objects[*a].get_bounds().center.y > bbox.center.y,
            axis::z => list.objects[*a].get_bounds().center.z > bbox.center.z,
        });
        let leftindices = slice[p..].to_vec();
        let rightindices = slice[..p].to_vec();
        Some(bvhNode {
            hittable_list: list,
            indices: slice.to_owned(),
            bbox: bbox,
            left: Some(Box::new(
                bvhNode::from_HittableList(list, leftindices).unwrap(),
            )), // TODO, CALL THIS FN
            right: Some(Box::new(
                bvhNode::from_HittableList(list, rightindices).unwrap(),
            )), // TODO, CALL THIS FN
        })
    }
}
