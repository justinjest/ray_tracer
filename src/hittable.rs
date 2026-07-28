use crate::rtweekend::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Arc::new(Lambertian::new(Color::new(1.0, 0.0, 1.0))),
            u: 0.0,
            v: 0.0,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Translate {
            object: object.clone(),
            offset,
            bbox: &object.bounding_box() + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r: Ray = Ray::new_with_time(*r.origin() - self.offset, *r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
