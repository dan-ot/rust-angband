use super::Vector2f;

#[derive(Clone, Copy, Debug)]
pub struct Matrix2x2f {
    pub data: [[f32; 2]; 2]
}

impl Matrix2x2f {
    pub fn zero() -> Matrix2x2f {
        Matrix2x2f { data: [[0.0, 0.0], [0.0, 0.0]] }
    }

    pub fn identity() -> Matrix2x2f {
        Matrix2x2f { data: [[1.0, 0.0], [0.0, 1.0]] }
    }

    pub fn uniform_scale(scale: f32) -> Matrix2x2f {
        Matrix2x2f { data: [[scale, 0.0], [0.0, scale]] }
    }

    pub fn from_scale(scale: Vector2f) -> Matrix2x2f {
        Matrix2x2f { data: [[scale.x, 0.0],[0.0, scale.y]] }
    }

    pub fn transform(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: rhs.x * self.data[0][0] + rhs.y * self.data[1][0],
            y: rhs.x * self.data[0][1] + rhs.y * self.data[1][1]
        }
    }

    pub fn concatenate(self, rhs: Matrix2x2f) -> Matrix2x2f {
        let [[a, b],[c, d]] = self.data;
        let [[e, f], [g, h]] = rhs.data;
        Matrix2x2f { data: [
            [a*e + b*g, a*f + b*h],
            [c*e + d*g, c*f + d*h]
        ]}
    }

    pub fn is_identity(self) -> bool {
        self.data == [[1.0, 0.0], [0.0, 1.0]]
    }
}