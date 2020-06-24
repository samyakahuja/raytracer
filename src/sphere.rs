use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = f64::sqrt(discriminant);
            let t = (-half_b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
            let t = (-half_b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t, p, normal })
            }
        }
        None
    }
}
