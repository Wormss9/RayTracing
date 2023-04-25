mod camera;
mod material;
mod ray;
mod settings;
mod shape;
mod vector;

use std::{process::Command, sync::Arc, thread};

use ray::Ray;
use shape::HittableList;
use vector::Vector;

use crate::{
    camera::Camera,
    settings::{ASPECT_RATIO_X, ASPECT_RATIO_Y, SAMPLES, THREADS, WIDTH},
};

pub fn random_f64() -> f64 {
    rand::random()
}

fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Vector {
    if depth == 128 {
        return Vector::new(0., 0., 0.);
    }

    if let Some((shape, rec)) = world.hit(&ray, 0.001, f64::MAX) {
        let scatterer = shape.scatter(&ray, &rec);
        if let Some(scattered) = scatterer {
            return rec.material.get_attenuation() * ray_color(scattered, world, depth + 1);
        }
        return Vector::new(0.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(material::Lambertian::new(Vector::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(shape::Sphere::new(
        Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vector::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.6 {
                    // diffuse
                    let albedo = Vector::random() * Vector::random();
                    let sphere_material = Arc::new(material::Lambertian::new(albedo));
                    world.add(Arc::new(shape::Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.85 {
                    // metal
                    let albedo = Vector::random() / 2.0 + Vector::new(0.5, 0.5, 0.5);
                    let fuzz = random_f64() / 2.0;
                    let sphere_material = Arc::new(material::Metal::new(albedo, Some(fuzz)));
                    world.add(Arc::new(shape::Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(material::Dielectric::new(1.5));
                    world.add(Arc::new(shape::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(material::Dielectric::new(1.5));
    world.add(Arc::new(shape::Sphere::new(
        Vector::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(material::Lambertian::new(Vector::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(shape::Sphere::new(
        Vector::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(material::Metal::new(Vector::new(0.7, 0.6, 0.5), Some(0.0)));
    world.add(Arc::new(shape::Sphere::new(
        Vector::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Image
    let aspect_ratio = ASPECT_RATIO_X as f64 / ASPECT_RATIO_Y as f64;
    let image_width = WIDTH;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let upscaling = SAMPLES; //30;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vector::new(13.0, 2.0, 3.0);
    let lookat = Vector::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut lines: Vec<thread::JoinHandle<String>> = vec![];

    for j in (0..image_height).rev() {
        let new_camera = camera.clone();
        let new_world = world.clone();

        let wait_for = image_height - j - THREADS;

        // Don't look here
        if wait_for >= 0 {
            let thread = &lines[wait_for as usize];

            eprintln!(
                "{} lines remaining, {:.2}%",
                image_height - wait_for,
                wait_for as f64 / image_height as f64 * 100.0
            );

            while !thread.is_finished() {
                let mut child = Command::new("sleep").arg("1").spawn().unwrap();
                child.wait().unwrap();
            }
        }

        lines.push(thread::spawn(move || {
            render_line(
                image_width,
                image_height,
                upscaling,
                j,
                new_camera,
                new_world,
            )
        }));
    }
    for line in lines {
        let text = line.join().unwrap();
        print!("{}", text)
    }

    let elapsed = now.elapsed();
    eprintln!("Elapsed: {:.2?}", elapsed);
}

fn render_line(
    image_width: i32,
    image_height: i32,
    upscaling: i32,
    j: i32,
    camera: Camera,
    world: HittableList,
) -> String {
    let mut line = "".to_owned();
    for i in 0..image_width {
        let mut color = Vector::new(0.0, 0.0, 0.0);
        for x in 0..(upscaling * upscaling) {
            let u = (i as f64 + (x / upscaling) as f64 / (upscaling - 1) as f64 - 0.5)
                / (image_width as f64 - 1.0);
            let v = (j as f64 + (x % upscaling) as f64 / (upscaling - 1) as f64 - 0.5)
                / (image_height as f64 - 1.0);
            let ray = camera.get_ray(u, v);
            color += ray_color(ray, &world, 0);
        }

        line += &color.get_string(upscaling * upscaling);
        line += "\n";
    }
    line
}
