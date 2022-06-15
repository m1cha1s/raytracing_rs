use crate::{ray::Ray, vec3::Vec3};

pub struct Hit {
    pub hit: bool,
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

impl Hit {
    pub fn new(hit: bool, p: Vec3, normal: Vec3, t: f64) -> Hit {
        Hit { hit, p, normal, t }
    }

    pub fn miss() -> Hit {
        Hit::new(false, Vec3::zeros(), Vec3::zeros(), 0.)
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64) -> Hit;
}
