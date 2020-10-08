use crate::ray::Ray;
use crate::vectors::Vector3;
pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
}
impl Camera {
    pub fn new(
        origin: Vector3<f64>,
        target: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperature: f64,
        focal_distance: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let up = Vector3::up();
        let w = (origin - target).normalized();
        let u = (up.cross(&w)).normalized();
        let v = w.cross(&u);
        let horiz = focal_distance * viewport_width * u;
        let vert = focal_distance * viewport_height * v;
        Camera {
            origin: origin,
            horizontal: horiz,
            vertical: vert,
            lower_left_corner: origin - horiz / 2 - vert / 2 - focal_distance * w,
            u: u,
            v: v,
            w: w,
            lens_radius: aperature / 2.0,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vector3::<f64>::random_in_unit_disk();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
