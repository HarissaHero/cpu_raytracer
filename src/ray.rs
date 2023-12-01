use crate::{vector::Vec3, sphere::Sphere};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn get_intersection_point(&self, sphere: &Sphere) -> Option<Vec3> {
        let intersection = sphere.intersect(self);
        if intersection == None {
            return None;
        }
        Some(self.origin + self.direction * intersection.unwrap())
    }
}
