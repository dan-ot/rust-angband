use crate::math::{Matrix2x2f, Vector2f};

#[derive(Clone, Copy, Debug)]
pub struct LayoutTransform2d {
    pub scale: f32,
    pub translation: Vector2f
}

impl LayoutTransform2d {
    pub fn default() -> LayoutTransform2d {
        LayoutTransform2d { scale: 1.0, translation: Vector2f::zero() }
    }

    pub fn new(scale: f32, translation: Vector2f) -> LayoutTransform2d {
        LayoutTransform2d { scale, translation }
    }

    pub fn from(translation: Vector2f) -> LayoutTransform2d {
        LayoutTransform2d { scale: 1.0_f32, translation }
    }

    pub fn transform_point(self, point: Vector2f) -> Vector2f {
        point + (point * self.scale)
    }
    
    pub fn transform_vector(self, point: Vector2f) -> Vector2f {
        point + (point * self.scale)
    }

    pub fn concatenate(self, rhs: LayoutTransform2d) -> LayoutTransform2d {
        LayoutTransform2d { scale: self.scale * rhs.scale, translation: rhs.transform_point(self.translation) }
    }

    pub fn inverse(self) -> LayoutTransform2d {
        let inverse_scale = 1.0_f32 / self.scale;
        LayoutTransform2d { scale: inverse_scale, translation: (-self.translation) * inverse_scale}
    }
}

pub struct RenderTransform2d {
    pub matrix: Matrix2x2f,
    pub translation: Vector2f
}

impl RenderTransform2d {
    pub fn default() -> RenderTransform2d {
        RenderTransform2d { matrix: Matrix2x2f::identity(), translation: Vector2f::zero() }
    }

    pub fn concatenate(self, rhs: RenderTransform2d) -> RenderTransform2d {
        RenderTransform2d { 
            matrix: self.matrix.concatenate(rhs.matrix),
            translation: rhs.matrix.transform(self.translation) + rhs.translation
        }
    }

    pub fn is_identity(self) -> bool {
        self.matrix.is_identity() && self.translation == Vector2f::zero()
    }
}

impl From<LayoutTransform2d> for RenderTransform2d {
    fn from(value: LayoutTransform2d) -> Self {
        RenderTransform2d { 
            matrix: Matrix2x2f::uniform_scale(value.scale), 
            translation: value.translation 
        }
    }
}