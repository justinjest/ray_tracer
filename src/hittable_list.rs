use crate::rtweekend::*;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb {
                x: Interval::new(0.0, 0.0),
                y: Interval::new(0.0, 0.0),
                z: Interval::new(0.0, 0.0),
            },
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::new_from_box(&self.bbox, &object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }
        sum
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len();
        if int_size == 1 {
            return self.objects[0].random(origin);
        }
        self.objects[random_int(0, int_size - 1)].random(origin)
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        for object in &self.objects {
            let mut temp_rec = HitRecord::new();
            if object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
