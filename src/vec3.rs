use super::*;
use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> Vec3 {
        Vec3(random_double(), random_double(), random_double())
    }

    pub fn random_minmax(min: f64, max: f64) -> Vec3 {
        Vec3(
            random_double_minmax(min, max),
            random_double_minmax(min, max),
            random_double_minmax(min, max),
        )
    }

    pub fn assign(&mut self, v: &Vec3) {
        self.0 = v.0;
        self.1 = v.1;
        self.2 = v.2;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Unary -
impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// []
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(self: &Vec3, i: usize) -> &Self::Output {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(),
        }
    }
}

// [] mut
impl ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!(),
        }
    }
}

// +
impl ops::Add for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

// -
impl ops::Sub for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// * Vec3
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

// Vec3 * scalar
impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// scalar * Vec3
impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

// / Vec3
impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

// / scalar
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// +=
impl ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
        self.2 = self.2 + rhs.2;
    }
}

// -=
impl ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
        self.1 = self.1 - rhs.1;
        self.2 = self.2 - rhs.2;
    }
}

// *= Vec3
impl ops::MulAssign<&Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.0 = self.0 * rhs.0;
        self.1 = self.1 * rhs.1;
        self.2 = self.2 * rhs.2;
    }
}

// *= scalar
impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self.0 * rhs;
        self.1 = self.1 * rhs;
        self.2 = self.2 * rhs;
    }
}

// /= Vec3
impl ops::DivAssign<&Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Vec3) {
        self.0 = self.0 / rhs.0;
        self.1 = self.1 / rhs.1;
        self.2 = self.2 / rhs.2;
    }
}

// /= scalar
impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 = self.0 / rhs;
        self.1 = self.1 / rhs;
        self.2 = self.2 / rhs;
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(
        v1.1 * v2.2 - v1.2 * v2.1,
        -(v1.0 * v2.2 - v1.2 * v2.0),
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    let a = random_double_minmax(0.0, 2.0 * PI);
    let z = random_double_minmax(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_minmax(-1.0, 1.0),
            random_double_minmax(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_minmax(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0
    // In the same hemisphere as the normal
    {
        in_unit_sphere
    } else {
        -&in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * dot(v, n) * n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * &(uv + &(cos_theta * n));
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
    &r_out_perp + &r_out_parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz() {
        assert!(Vec3(0.0, 1.0, 2.0).x() == 0.0);
        assert!(Vec3(0.0, 1.0, 2.0).y() == 1.0);
        assert!(Vec3(0.1, 1.0, 2.0).z() == 2.0);
    }

    #[test]
    fn test_neg() {
        assert!(-Vec3(0.1, 0.2, 0.3).0 == -0.1);
        assert!(-Vec3(0.1, 0.2, 0.3).1 == -0.2);
        assert!(-Vec3(0.1, 0.2, 0.3).2 == -0.3);
        assert!(-(-&Vec3(0.1, 0.2, 0.3)).0 == 0.1);
        assert!(-(-&Vec3(0.1, 0.2, 0.3)).1 == 0.2);
        assert!(-(-&Vec3(0.1, 0.2, 0.3)).2 == 0.3);

        let v = Vec3(0.1, 0.2, 0.3);
        let v1 = -&v;
        let v2 = -&v1;
        assert!(v.0 == -v1.0);
        assert!(v.0 == v2.0);
    }
    #[test]
    fn test_index() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v = v0.clone();
        let e0 = v[0];
        let e1 = v[1];
        let e2 = v[2];
        assert!(e0 == v0[0]);
        assert!(e1 == v0[1]);
        assert!(e2 == v0[2]);
    }
    #[test]
    fn test_index_mut() {
        let mut v = Vec3(0.1, 0.2, 0.3);
        let e0 = v[0];
        let e1 = v[1];
        let e2 = v[2];
        v[0] = 0.4;
        v[1] = 0.5;
        v[2] = 0.6;
        assert!(e0 == 0.1);
        assert!(e1 == 0.2);
        assert!(e2 == 0.3);
        assert!(v[0] == 0.4);
        assert!(v[1] == 0.5);
        assert!(v[2] == 0.6);
    }

    #[test]
    fn test_add() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        let v2 = Vec3(1.0, 2.0, 3.0);
        v1 = &v1 + &v2;
        assert!(v1.0 == v0.0 + v2.0);
        assert!(v1.1 == v0.1 + v2.1);
        assert!(v1.2 == v0.2 + v2.2);
    }

    #[test]
    fn test_sub() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        let v2 = Vec3(1.0, 2.0, 3.0);
        v1 = &v1 - &v2;
        assert!(v1.0 == v0.0 - v2.0);
        assert!(v1.1 == v0.1 - v2.1);
        assert!(v1.2 == v0.2 - v2.2);
    }
    #[test]
    fn test_mul_vec3() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        let v2 = Vec3(1.0, 2.0, 3.0);
        v1 = &v1 * &v2;
        assert!(v1.0 == v0.0 * v2.0);
        assert!(v1.1 == v0.1 * v2.1);
        assert!(v1.2 == v0.2 * v2.2);
    }

    #[test]
    fn test_mul_scalar() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        v1 = &v1 * 2.0;
        assert!(v1.0 == v0.0 * 2.0);
        assert!(v1.1 == v0.1 * 2.0);
        assert!(v1.2 == v0.2 * 2.0);
    }

    #[test]
    fn test_mul_scalar2() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        v1 = 2.0 * &v1;
        assert!(v1.0 == v0.0 * 2.0);
        assert!(v1.1 == v0.1 * 2.0);
        assert!(v1.2 == v0.2 * 2.0);
    }

    #[test]
    fn test_div_vec3() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        let v2 = Vec3(1.0, 2.0, 3.0);
        v1 = &v1 / &v2;
        assert!(v1.0 == v0.0 / v2.0);
        assert!(v1.1 == v0.1 / v2.1);
        assert!(v1.2 == v0.2 / v2.2);
    }

    #[test]
    fn test_div_scalar() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let mut v1 = v0.clone();
        v1 = &v1 / 2.0;
        assert!(v1.0 == v0.0 / 2.0);
        assert!(v1.1 == v0.1 / 2.0);
        assert!(v1.2 == v0.2 / 2.0);
    }

    #[test]
    fn test_add_assign() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = Vec3(1.1, 1.2, 1.3);
        let mut v2 = v0.clone();
        v2 += v1.clone();
        assert!(v2.0 == v0.0 + v1.0);
        assert!(v2.1 == v0.1 + v1.1);
        assert!(v2.2 == v0.2 + v1.2);
    }

    #[test]
    fn test_sub_assign() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = Vec3(1.1, 1.2, 1.3);
        let mut v2 = v0.clone();
        v2 -= v1.clone();
        assert!(v2.0 == v0.0 - v1.0);
        assert!(v2.1 == v0.1 - v1.1);
        assert!(v2.2 == v0.2 - v1.2);
    }

    #[test]
    fn test_mul_assign_vec3() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = Vec3(1.1, 1.2, 1.3);
        let mut v2 = v0.clone();
        v2 *= &v1;
        assert!(v2.0 == v0.0 * v1.0);
        assert!(v2.1 == v0.1 * v1.1);
        assert!(v2.2 == v0.2 * v1.2);
    }

    #[test]
    fn test_mul_assign_scalar() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = 2.0;
        let mut v2 = v0.clone();
        v2 *= v1;
        assert!(v2.0 == v0.0 * v1);
        assert!(v2.1 == v0.1 * v1);
        assert!(v2.2 == v0.2 * v1);
    }

    #[test]
    fn test_div_assign_vec3() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = Vec3(1.1, 1.2, 1.3);
        let mut v2 = v0.clone();
        v2 /= &v1;
        assert!(v2.0 == v0.0 / v1.0);
        assert!(v2.1 == v0.1 / v1.1);
        assert!(v2.2 == v0.2 / v1.2);
    }

    #[test]
    fn test_div_assign_scalar() {
        let v0 = Vec3(0.1, 0.2, 0.3);
        let v1 = 2.0;
        let mut v2 = v0.clone();
        v2 /= v1;
        assert!(v2.0 == v0.0 / v1);
        assert!(v2.1 == v0.1 / v1);
        assert!(v2.2 == v0.2 / v1);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3(0.1, 0.2, 0.3);
        assert!(v.length_squared() == (0.1_f64 * 0.1_f64 + 0.2_f64 * 0.2_f64 + 0.3_f64 * 0.3_f64));
    }

    #[test]
    fn test_length() {
        let v = Vec3(0.1, 0.2, 0.3);
        assert!(v.length() == (0.1_f64 * 0.1_f64 + 0.2_f64 * 0.2_f64 + 0.3_f64 * 0.3_f64).sqrt());
    }

    #[test]
    fn test_dot() {
        let v = Vec3(0.1, 0.2, 0.3);
        let v2 = Vec3(2.0, 3.0, 4.0);
        let d = dot(&v, &v2);
        assert!(d == v.0 * v2.0 + v.1 * v2.1 + v.2 * v2.2);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3(0.1, 0.2, 0.3);
        let v2 = Vec3(2.0, 3.0, 4.0);
        let d = cross(&v1, &v2);
        assert!(d.0 == v1.1 * v2.2 - v1.2 * v2.1);
        assert!(d.1 == -(v1.0 * v2.2 - v1.2 * v2.0));
        assert!(d.2 == v1.0 * v2.1 - v1.1 * v2.0);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3(0.1, 2.0, 1.0);
        let uv = unit_vector(&v);
        assert!(uv.length() == 1.0);
    }
}
