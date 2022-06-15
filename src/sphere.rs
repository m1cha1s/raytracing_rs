use crate::{
    hit::{Hit, Hittable},
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Hit {
        let oc = r.orig - self.center;
        let a = r.dir.lenght_squared();
        let half_b = Vec3::dot(oc, r.dir);
        let c = oc.lenght_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return Hit::miss();
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < ray_tmin || ray_tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < ray_tmin || ray_tmax < root {
                return Hit::miss();
            }
        }

        Hit::new(
            true,
            r.at(root),
            (r.at(root) - self.center) / self.radius,
            root,
        )
    }
}
