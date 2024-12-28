use approx::{AbsDiffEq, RelativeEq, UlpsEq};

pub type Float = f64;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const D: usize>(pub [Float; D]);

impl<const D: usize> Default for Vector<D> {
    fn default() -> Self {
        Self([0.0; D])
    }
}

// implement addition for vectors
impl<const D: usize> std::ops::Add for Vector<D> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..D {
            result.0[i] += other.0[i];
        }
        result
    }
}

// implement subtraction for vectors
impl<const D: usize> std::ops::Sub for Vector<D> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..D {
            result.0[i] -= other.0[i];
        }
        result
    }
}

// implement vector - scalar product
impl<const D: usize> std::ops::Mul<Float> for Vector<D> {
    type Output = Self;

    fn mul(self, scalar: Float) -> Self {
        let mut result = self;
        for i in 0..D {
            result.0[i] *= scalar;
        }
        result
    }
}

// implement scalar - vector product
impl<const D: usize> std::ops::Mul<Vector<D>> for Float {
    type Output = Vector<D>;

    fn mul(self, vector: Vector<D>) -> Vector<D> {
        vector * self
    }
}

// implement vector - scalar division
impl<const D: usize> std::ops::Div<Float> for Vector<D> {
    type Output = Self;

    fn div(self, scalar: Float) -> Self {
        let mut result = self;
        for i in 0..D {
            result.0[i] /= scalar;
        }
        result
    }
}

// implement equality
impl<const D: usize> std::cmp::PartialEq for Vector<D> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<const D: usize> AbsDiffEq for Vector<D> {
    type Epsilon = Float;

    fn default_epsilon() -> Self::Epsilon {
        Float::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

impl<const D: usize> RelativeEq for Vector<D> {
    fn default_max_relative() -> Self::Epsilon {
        Float::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

impl<const D: usize> UlpsEq for Vector<D> {
    fn default_max_ulps() -> u32 {
        Float::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}
