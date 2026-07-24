use crate::rtweekend::*;

pub fn rotate(object: Arc<dyn Hittable>, rotation: Vec3) -> Arc<dyn Hittable> {
    Arc::new(RotateZ::new(
        Arc::new(RotateY::new(
            Arc::new(RotateX::new(object, rotation.x())),
            rotation.y(),
        )),
        rotation.z(),
    ))
}

struct RotateZ {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateZ {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;
                    let x = i_f * bbox.x.max + (1.0 - i_f) * bbox.x.min;
                    let y = j_f * bbox.y.max + (1.0 - j_f) * bbox.y.min;
                    let z = k_f * bbox.z.max + (1.0 - k_f) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * y;
                    let new_y = -sin_theta * x + cos_theta * y;

                    let tester = Point3::new(new_x, new_y, z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        RotateZ {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(&min, &max),
        }
    }
}

impl Hittable for RotateZ {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().y()),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().y()),
            r.origin().z(),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().y()),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().y()),
            r.direction().z(),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.y()),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.y()),
            rec.p.z(),
        );

        rec.normal = Point3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.y()),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.y()),
            rec.normal.z(),
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

struct RotateX {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateX {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;
                    let x = i_f * bbox.x.max + (1.0 - i_f) * bbox.x.min;
                    let y = j_f * bbox.y.max + (1.0 - j_f) * bbox.y.min;
                    let z = k_f * bbox.z.max + (1.0 - k_f) * bbox.z.min;

                    let new_y = cos_theta * y - sin_theta * z;
                    let new_z = sin_theta * y + cos_theta * z;

                    let tester = Point3::new(x, new_y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        RotateX {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(&min, &max),
        }
    }
}

impl Hittable for RotateX {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            r.origin().x(),
            (self.cos_theta * r.origin().y()) - (self.sin_theta * r.origin().z()),
            (self.sin_theta * r.origin().y()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            r.direction().x(),
            (self.cos_theta * r.direction().y()) - (self.sin_theta * r.direction().z()),
            (self.sin_theta * r.direction().y()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            rec.p.x(),
            (self.cos_theta * rec.p.y()) - (self.sin_theta * rec.p.z()),
            (self.sin_theta * rec.p.y()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Point3::new(
            rec.normal.x(),
            (self.cos_theta * rec.normal.y()) - (self.sin_theta * rec.normal.z()),
            (self.sin_theta * rec.normal.y()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;
                    let x = i_f * bbox.x.max + (1.0 - i_f) * bbox.x.min;
                    let y = j_f * bbox.y.max + (1.0 - j_f) * bbox.y.min;
                    let z = k_f * bbox.z.max + (1.0 - k_f) * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Point3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Point3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
