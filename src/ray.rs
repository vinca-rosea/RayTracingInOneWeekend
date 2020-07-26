use super::Point3;
use super::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Default::default(),
            dir: Default::default(),
            tm: Default::default(),
        }
    }
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Ray { orig, dir, tm }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        &self.orig + &(t * &self.dir)
    }

    pub fn assign(&mut self, r: &Ray) {
        self.orig = r.orig.clone();
        self.dir = r.dir.clone();
        self.tm = r.tm;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         let r = Ray::new(Point3::new(0.0, 0.1, 0.2), Vec3(1.0, 1.1, 1.2));
//         assert_eq!(r.origin().0, 0.0);
//         assert_eq!(r.direction().0, 1.0);
//     }

//     #[test]
//     fn test_origin() {
//         let r = Ray::new(Point3::new(0.0, 0.1, 0.2), Vec3(1.0, 1.1, 1.2));
//         assert_eq!(r.origin().0, 0.0);
//         assert_eq!(r.origin().1, 0.1);
//         assert_eq!(r.origin().2, 0.2);
//     }

//     #[test]
//     fn test_direction() {
//         let r = Ray::new(Point3::new(0.0, 0.1, 0.2), Vec3(1.0, 1.1, 1.2));
//         assert_eq!(r.direction().0, 1.0);
//         assert_eq!(r.direction().1, 1.1);
//         assert_eq!(r.direction().2, 1.2);
//     }

//     #[test]
//     fn test_at() {
//         let o = Vec3(0.0, 0.1, 0.2);
//         let d = Vec3(1.0, 1.1, 1.2);
//         let r = Ray::new(o.clone(), d.clone());
//         let v = r.at(2.0);

//         assert_eq!(v.0, o.0 + 2.0 * d.0);
//         assert_eq!(v.1, o.1 + 2.0 * d.1);
//         assert_eq!(v.2, o.2 + 2.0 * d.2);
//     }
// }
