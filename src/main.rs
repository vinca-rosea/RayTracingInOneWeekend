mod camera;
mod color;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::*;
use color::*;
use hitable::*;
use hitable_list::*;
use material::*;
use rand::Rng;
use ray::*;
use rayon::prelude::*;
use rtweekend::*;
use sphere::*;
use std::io::Write;
use std::sync::Arc;
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hitable, depth: i64) -> Color {
    let mut rec: HitRecord = Default::default();
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return &attenuation * &ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    &((1.0 - t) * &Color::new(1.0, 1.0, 1.0)) + &(t * &Color::new(0.5, 0.7, 1.0))
}

fn random_scene() -> HitableList {
    let mut world = HitableList::new();
    //let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_material = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = &Color::random() * &Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_minmax(0.5, 1.0);
                    let fuzz = random_double_minmax(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() -> std::io::Result<()> {
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 1200;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 500;
    const MAX_DEPTH: i64 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        0.0,
    );

    // Render
    writeln!(out, "P3")?;
    writeln!(out, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(out, "255")?;

    let mut result: Vec<Vec<(i64, i64, i64)>> = Vec::with_capacity(IMAGE_HEIGHT as usize);
    (0..IMAGE_HEIGHT as i32)
        .into_par_iter()
        .rev()
        .map(|j| {
            let mut err = std::io::stderr();

            writeln!(err, "Scanlines remaining: {} ", j);
            let mut color: Vec<(i64, i64, i64)> = Vec::with_capacity(IMAGE_WIDTH as usize);

            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for s in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                color.push(get_color(pixel_color, SAMPLES_PER_PIXEL));
            }
            color
        })
        .collect_into_vec(&mut result);

    let mut err = std::io::stderr();
    writeln!(err, "File output start.")?;
    result.iter().for_each(|row| {
        row.iter().for_each(|col| {
            writeln!(out, "{} {} {}", col.0, col.1, col.2,);
        });
    });

    writeln!(err, "Done.")?;
    Ok(())
}
