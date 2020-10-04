use crate::ray::Ray;
use crate::vectors::Vector3;
pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}
impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let o = Vector3::zero();
        let h = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let v = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        Camera {
            origin: o,
            horizontal: h,
            vertical: v,
            lower_left_corner: o
                - h / 2
                - v / 2
                - (Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: focal_length,
                }),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
