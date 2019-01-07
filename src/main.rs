use std::f32;
use std::fs;
use std::io::{BufWriter, Write};

extern crate rand;
use rand::distributions::{Distribution, Normal};
use rand::prelude::random;

mod vector;
mod ray;
mod sphere;
mod hitable;
mod world;
mod camera;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::world::World;
use crate::hitable::{HitableRecord, Hitable};
use crate::camera::Camera;

fn random_in_unit_sphere() -> Vec3 {
    let mut p = 2.0 * Vec3::new(drand(), drand(), drand())
        - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(drand(), drand(), drand())
            - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn drand() -> f32 {
    random::<f32>()
}

fn color(r: &Ray, world: &World) -> Vec3 {
    let mut rec = HitableRecord::new();
    if world.hit(r, 0.001, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(&rec.p, &(target - rec.p)), &world);
    }
    let unit_direction = Vec3::unit_vec(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let nx = 600;
    let ny = 300;
    let ns = 100;

    let file = fs::File::create("image.ppm")?;
    let mut buffer = BufWriter::new(file);
    
    buffer.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny))?;

    let mut world = World::new();
    world.add(Sphere::new(0.0, 0.0, -1.0, 0.5));
    world.add(Sphere::new(0.0, -100.5, -1.0, 100.0));

    let cam = Camera::new();

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + drand()) / nx as f32;
                let v = (j as f32 + drand()) / ny as f32;

                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }

            col /= ns as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            buffer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }
    
    Ok(())
}