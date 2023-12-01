use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r == other.r)
            && (self.g == other.g)
            && (self.b == other.b)
            && (self.alpha == other.alpha)
    }
}

impl Eq for Color {}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: (self.r as f64 * rhs).round() as u8,
            g: (self.g as f64 * rhs).round() as u8,
            b: (self.b as f64 * rhs).round() as u8,
            alpha: self.alpha,
        }
    }
}


