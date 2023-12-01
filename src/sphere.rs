use crate::{color::Color, light::Light, material::Material, ray::Ray, vector::Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        (self.center == other.center)
            && self.radius == other.radius
            && self.material == other.material
    }
}

impl Eq for Sphere {}

impl Sphere {
    pub fn intersect(&self, Ray { origin, direction }: &Ray) -> Option<f64> {
        let oc = *origin - self.center;
        let a = direction.dot(&direction);
        let b = 2.0 * oc.dot(&direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
            if t1 > 0.0 && t2 > 0.0 {
                return Some(t1.min(t2));
            } else if t1 > 0.0 {
                return Some(t1);
            } else if t2 > 0.0 {
                return Some(t2);
            } else {
                return None;
            }
        }
    }

    pub fn get_normal_vec_at(&self, point: &Vec3) -> Vec3 {
        &(*point - self.center) / self.center.get_distance_from(point)
    }

    pub fn get_brightness_at(
        &self,
        point: &Vec3,
        light: &Light,
        objects: &Vec<Sphere>,
    ) -> Option<f64> {
        let direction_light_to_point = point.get_direction_to(&light.origin);
        for object in objects {
            match object.intersect(&Ray {
                origin: *point,
                direction: direction_light_to_point,
            }) {
                Some(_) => {
                    return None;
                }
                None => {}
            }
        }
        let normal_vec_at_point = self.get_normal_vec_at(&point);
        let t = direction_light_to_point.dot(&normal_vec_at_point).max(0.0);
        Some(t)
    }

    pub fn get_color_at(&self, point: &Vec3, light: &Light, objects: &Vec<Sphere>) -> Color {
        self.material.albedo * self.get_brightness_at(point, light, objects).unwrap_or(0.0)
    }
}
