use std::ops;
use rand::Rng;
use super::{random_f32, EPSILON};

#[derive(Copy, Clone)]
/// # `Vector3`
/// Vector in 3-dimensional space. Includes algebraic functions such as dot, cross, norm
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    /// # `new`
    /// Returns a `Vector3` by taking its given corresponding x, y, z components as `f32`
    pub fn new(x: f32, y:f32, z:f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    /// # `dot`
    /// Returns as `f32` the dot product of this `Vector3` and another given `Vector3`.
    pub fn dot(&self, _other: Vector3) -> f32 {
        self.x * _other.x +
        self.y * _other.y +
        self.z * _other.z
    }

    /// # `cross`
    /// Returns as `Vector3` the cross product of this `Vector3` x the given `Vector3`
    pub fn cross(&self, _other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * _other.z - self.z * _other.y,
            y: self.z * _other.x - self.x * _other.z,
            z: self.x * _other.y - self.y * _other.x
        }
    }

    /// # `norm`
    /// Returns as `f32` the norm ||Vector|| of this vector 
    pub fn norm(&self) -> f32 {
        (self.x * self.x + 
         self.y * self.y + 
         self.z * self.z).sqrt()
    }

    /// # `norm_squared`
    /// Returns as `f32` the norm squared ||Vector||^2 of this vector 
    pub fn norm_squared(&self) -> f32 {
        self.x * self.x + 
        self.y * self.y + 
        self.z * self.z
    }

    /// # `unit`
    /// Returns as `Vector3` a vector of length one with the same direction as this `Vector3`
    pub fn unit(&self) -> Vector3 {
        let norm : f32 = self.norm();
        Vector3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        }
    }

    /// # `random`
    /// Creates a `Vector3` with components of random size in range [0, 1)
    pub fn random() -> Vector3 {
        Vector3 {
            x: random_f32(),
            y: random_f32(),
            z: random_f32()
        }
    }

    /// # `random_bounded`
    /// Creates a `Vector3` with components of random size in given range as `f32` range [min, max]
    pub fn random_bounded(min: f32, max: f32) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max)
        }
    }

    /// # `random_in_unit_sphere`
    /// Creates a random `Vector3` that is within a unit circle
    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let vect = Vector3::random_bounded(-1.0, 1.0);
            if vect.norm_squared() >= 1.0 {continue;}
            return vect;
        }
    }

    /// # `near_zero`
    /// Returns true if vector is very close to zero
    pub fn near_zero(&self) -> bool {
        self.x.abs() < EPSILON && self.y < EPSILON && self.z < EPSILON
    }

    /// # `refract`
    /// Returns the refraction of this vector as `Vector3` using Snell's Law of Refraction and
    /// using the given normal vector `Vector3` and the index of refraction as `f32`.
    pub fn refract(&self, normal: Vector3, index: f32) -> Vector3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let ray_perp = index * (*self + cos_theta * normal);
        let ray_paral = - ((1.0 - ray_perp.norm_squared()).abs()).sqrt() * normal;

        ray_perp + ray_paral
    }

    /// # `reflect`
    /// Returns a reflection of this `Vector3` around a given normal `Vector3`
    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        *self - (2.0 * (*self).dot(normal)) * normal
    }
}

// Vector + Vector
impl ops::Add<Vector3> for Vector3 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

// Vector - Vector 
impl ops::Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

// Vector * scalar (`f32`) 
impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, _rhs: f32) -> Self {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

// scalar (`f32`) * Vector
impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z
        }
    }
}

// Vector * Vector (component * component)
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3; 

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z
        }
    }
}

// Vector / scalar (`f32`)
impl ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, _rhs: f32) -> Self {
        Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs
        }
    }
}

// -Vector
impl ops::Neg<> for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

// Vector3 += Vector3
impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

// Vector3 -= Vector3
impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, _rhs: Self) {
        *self = Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

// Vector3 *= Scalar `f32`
impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, _rhs: f32) {
        *self = Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

// Vector3 /= Scalar `f32`
impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, _rhs: f32) {
        *self = Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs
        }
    }
}