use crate::rtweekend::*;
use std::cmp::Ordering;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    fn new_from_slice(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        let mut bbox = objects[start].bounding_box();

        for obj in &objects[start + 1..end] {
            bbox = Aabb::new_from_box(&bbox, &obj.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparater = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Crash in BvhNode"),
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = if object_span == 1 {
            let left = objects[start].clone();
            let right = left.clone();
            (left, right)
        } else if object_span == 2 {
            let left = objects[start].clone();
            let right = objects[start + 1].clone();
            (left, right)
        } else {
            objects[start..end].sort_by(comparater);
            let mid = start + object_span / 2;
            let left: Arc<dyn Hittable> = Arc::new(Self::new_from_slice(objects, start, mid));
            let right: Arc<dyn Hittable> = Arc::new(Self::new_from_slice(objects, mid, end));
            (left, right)
        };

        let bbox = Aabb::new_from_box(&left.bounding_box(), &right.bounding_box());
        BvhNode { left, right, bbox }
    }

    pub fn new(mut list: HittableList) -> Self {
        let end = list.objects.len();
        Self::new_from_slice(&mut list.objects, 0, end)
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: u64) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval
        .min
        .partial_cmp(&b_axis_interval.min)
        .unwrap_or(Ordering::Equal)
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &ray_t.clone()) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let tmp = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, tmp), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
