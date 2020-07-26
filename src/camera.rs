use super::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            40.0,
            1.0,
            0.0,
            10.0,
            0.0,
            0.0,
        )
    }
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(&lookfrom - &lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner =
            &(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &(focus_dist * &w);

        let lens_radius = aperture / 2.0;
        let time0 = t0;
        let time1 = t1;

        Self {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            time0: time0,
            time1: time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * &random_in_unit_disk();
        let offset = &(&self.u * rd.x()) + &(&self.v * rd.y());
        Ray::new(
            &self.origin + &offset,
            &(&(&(&self.lower_left_corner + &(s * &self.horizontal)) + &(t * &self.vertical))
                - &self.origin)
                - &offset,
            random_double_minmax(self.time0, self.time1),
        )
    }
}
