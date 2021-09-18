use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{dot, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    pub pos: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            pos: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub struct HittableList {
    list: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: vec![] }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_record = HitRecord::new();

        for object in &self.list {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                record.t = temp_record.t;
                record.front_face = temp_record.front_face;
                record.normal = temp_record.normal;
                record.pos = temp_record.pos;
                record.material = temp_record.material.clone();
            }
        }
        hit_anything
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.list.push(object);
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    centor: Vec3,
    radius: f64,
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(centor: Vec3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            centor: centor,
            radius: radius,
            material: Some(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let normalized_centor = ray.orig - self.centor;
        let a = dot(ray.dir, ray.dir);
        let b = dot(ray.dir, normalized_centor);
        let c = dot(normalized_centor, normalized_centor) - self.radius * self.radius;

        let discriminat = b * b - a * c;
        if discriminat < 0.0 {
            return false;
        }

        let sqrtd = discriminat.sqrt();
        let mut root = (-b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.pos = ray.at(root);
        let outward_normal = (record.pos - self.centor) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = self.material.clone();

        return true;
    }
}
