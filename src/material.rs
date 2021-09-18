use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::Rand;
use crate::vector::{dot, reflect, refract, Color, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rand: &mut Rand,
    ) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rand: &mut Rand,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector(rand);
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        *scattered = Ray::new(record.pos, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rand: &mut Rand,
    ) -> bool {
        let reflected = reflect(r_in.dir.unit_vector(), record.normal)
            + Vec3::random_unit_vector(rand) * self.fuzz;
        *scattered = Ray::new(record.pos, reflected);
        *attenuation = self.albedo;
        dot(scattered.dir, record.normal) > 0.0
    }
}

pub struct Glass {
    albedo: Color,
    eta: f64,
}

impl Glass {
    pub fn new(albedo: Color, eta: f64) -> Glass {
        Glass {
            albedo: albedo,
            eta: eta,
        }
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rand: &mut Rand,
    ) -> bool {
        let refraction_ratio = if record.front_face {
            1.0 / self.eta
        } else {
            self.eta
        };

        let cos_theta = dot(-r_in.dir.unit_vector(), record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0 * r0;
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }

        let refracted: Vec3;
        if refraction_ratio * sin_theta > 1.0
            || reflectance(cos_theta, refraction_ratio) > rand.next_double()
        {
            refracted = reflect(r_in.dir.unit_vector(), record.normal);
        } else {
            refracted = refract(r_in.dir.unit_vector(), record.normal, refraction_ratio);
        }

        *scattered = Ray::new(record.pos, refracted);
        *attenuation = self.albedo;
        true
    }
}
