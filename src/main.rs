mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vector;
use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere};
use crate::material::{Glass, Lambertian, Metal};
use std::rc::Rc;
use util::Rand;
use vector::{Color, Vec3};

fn random_scene(rand: &mut Rand) -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rand.next_double();
            let center = Vec3::new(
                i as f64 + 0.9 * rand.next_double(),
                0.2,
                j as f64 + 0.9 * rand.next_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rand) * Color::random(rand);
                    let sphere_material = Rc::new(Lambertian::new(albedo));

                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }

                if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(rand);
                    let fuzz = rand.next_specified_double(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));

                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }

                if choose_mat < 0.8 {
                    // glass
                    let albedo = Color::new(1.0, 1.0, 1.0);
                    let sphere_material = Rc::new(Glass::new(albedo, 1.5));

                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Glass::new(Color::new(1.0, 1.0, 1.0), 1.5));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let mut rand = Rand::new();

    // World
    let world = random_scene(&mut rand);

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 * aspect_ratio) as usize;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
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
    let samples_per_pixel = 500;
    let max_depth = 50;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand.next_double()) / (image_width - 1) as f64;
                let v = (j as f64 + rand.next_double()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v, &mut rand);
                pixel_color += ray.ray_color(&world, max_depth, &mut rand);
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }
}
