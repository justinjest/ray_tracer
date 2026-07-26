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

                    let new_x = cos_theta * x - sin_theta * y;
                    let new_y = sin_theta * x + cos_theta * y;

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
            (self.cos_theta * r.origin().x()) + (self.sin_theta * r.origin().y()),
            (-self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().y()),
            r.origin().z(),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) + (self.sin_theta * r.direction().y()),
            (-self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().y()),
            r.direction().z(),
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) - (self.sin_theta * rec.p.y()),
            (self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.y()),
            rec.p.z(),
        );

        rec.normal = Point3::new(
            (self.cos_theta * rec.normal.x()) - (self.sin_theta * rec.normal.y()),
            (self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.y()),
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
            (self.cos_theta * r.origin().y()) + (self.sin_theta * r.origin().z()),
            (-self.sin_theta * r.origin().y()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            r.direction().x(),
            (self.cos_theta * r.direction().y()) + (self.sin_theta * r.direction().z()),
            (-self.sin_theta * r.direction().y()) + (self.cos_theta * r.direction().z()),
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

pub struct Scale {
    object: Arc<dyn Hittable>,
    scale: Vec3,
    inv_scale: Vec3,
}

impl Scale {
    pub fn new(object: Arc<dyn Hittable>, scale: Vec3) -> Self {
        Self {
            object,
            scale,
            inv_scale: Vec3 {
                x: 1.0 / scale.x(),
                y: 1.0 / scale.y(),
                z: 1.0 / scale.z(),
            },
        }
    }
}

impl Hittable for Scale {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let local_origin: Vec3 = *r.origin() * self.inv_scale;
        let local_direction: Vec3 = *r.direction() * self.inv_scale;
        let local_ray = Ray::new_with_time(local_origin, local_direction, r.time());

        if !self.object.hit(&local_ray, ray_t, rec) {
            return false;
        }

        rec.p = rec.p * self.scale;
        rec.t = (rec.p - *r.origin()).length() / r.direction().length();
        // I don't think this will work for spheres
        rec.normal = unit_vector(rec.normal * self.inv_scale);
        true
    }

    fn bounding_box(&self) -> Aabb {
        let bbox = self.object.bounding_box();
        let min = Vec3::new(bbox.x.min, bbox.y.min, bbox.z.min);
        let max = Vec3::new(bbox.x.max, bbox.y.max, bbox.z.max);
        let scaled_min = min * self.scale;
        let scaled_max = max * self.scale;

        Aabb::new_from_points(
            &Point3::new(
                scaled_min.x.min(scaled_max.x),
                scaled_min.y.min(scaled_max.y),
                scaled_min.z.min(scaled_max.z),
            ),
            &Point3::new(
                scaled_min.x.max(scaled_max.x),
                scaled_min.y.max(scaled_max.y),
                scaled_min.z.max(scaled_max.z),
            ),
        )
    }
}
