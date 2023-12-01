use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub albedo: Color,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.albedo == other.albedo
    }
}

impl Eq for Material {}


