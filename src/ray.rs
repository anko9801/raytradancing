use crate::hittable::{HitRecord, HittableList};
use crate::util::Rand;
use crate::vector::{Color, Vec3};

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }

    pub fn ray_color(&self, world: &HittableList, depth: isize, rand: &mut Rand) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // diffuse
        let mut record = HitRecord::new();
        if world.hit(self, 0.001, f64::MAX, &mut record) {
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            if let Some(mat) = &record.material {
                mat.scatter(self, &record, &mut attenuation, &mut scattered, rand);
            }
            return attenuation * scattered.ray_color(world, depth - 1, rand);
        }

        let unit_direction = self.dir.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }
}
