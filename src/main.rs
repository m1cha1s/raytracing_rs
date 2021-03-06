use raylib::prelude::*;
use vec3::Vec3;

mod hit;
mod ray;
mod sphere;
mod vec3;

fn hit_sphere(center: Vec3, radius: f64, r: ray::Ray) -> f64 {
    let oc = r.orig - center;
    let a = r.dir.lenght_squared();
    let half_b = Vec3::dot(oc, r.dir);
    let c = oc.lenght_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: ray::Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0., 0., -1.), 0.5, ray.clone());
    if t > 0. {
        let n = Vec3::normalize(ray.at(t) - Vec3::new(0., 0., -1.));
        return Vec3::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5;
    }
    let unit_direction = Vec3::normalize(ray.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (Vec3::new(1., 1., 1.)) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16. / 9.;
    let width: i32 = 800;
    let height: i32 = (width as f64 / aspect_ratio) as i32;

    // Camera

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_lenght: f64 = 1.0;

    let origin: Vec3 = Vec3::new(0., 0., 0.);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner: Vec3 =
        origin - (horizontal / 2.) - (vertical / 2.) - Vec3::new(0., 0., focal_lenght);

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raytracing")
        .build();

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLACK);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let r: ray::Ray = ray::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let col = ray_color(r);
            d.draw_pixel(
                i,
                height - j - 1,
                Color::new(
                    (col.x * 255.) as u8,
                    (col.y * 255.) as u8,
                    (col.z * 255.) as u8,
                    255,
                ),
            )
        }
    }

    drop(d);

    while !rl.window_should_close() {
        let _d = rl.begin_drawing(&thread);
    }
}
