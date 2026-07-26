use crate::rtweekend::*;
use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Aabb {
        let mut aabb = Aabb {
            x: *x,
            y: *y,
            z: *z,
        };
        aabb.pad_to_minimus();
        aabb
    }

    pub fn empty() -> Aabb {
        let empty = Interval { min: 0.0, max: 0.0 };
        Aabb {
            x: empty,
            y: empty,
            z: empty,
        }
    }

    pub fn new_from_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let x = Interval::tight_expansion(&box0.x, &box1.x);
        let y = Interval::tight_expansion(&box0.y, &box1.y);
        let z = Interval::tight_expansion(&box0.z, &box1.z);
        Aabb { x, y, z }
    }

    pub fn new_from_points(a: &Point3, b: &Point3) -> Aabb {
        let mut aabb = Aabb {
            x: Interval {
                min: f64::min(a.x(), b.x()),
                max: f64::max(a.x(), b.x()),
            },
            y: Interval {
                min: f64::min(a.y(), b.y()),
                max: f64::max(a.y(), b.y()),
            },
            z: Interval {
                min: f64::min(a.z(), b.z()),
                max: f64::max(a.z(), b.z()),
            },
        };
        aabb.pad_to_minimus();
        aabb
    }

    fn pad_to_minimus(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta)
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta)
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta)
        }
    }

    pub fn axis_interval(&self, n: u64) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        let ray_origin = r.origin();
        let inv_ray_direction = r.invdir();

        for a in 0..3 {
            let ax = self.axis_interval(a as u64);
            let adinv = inv_ray_direction[a];

            let t0 = (ax.min - ray_origin[a]) * adinv;
            let t1 = (ax.max - ray_origin[a]) * adinv;

            if t0 < t1 {
                if t0 > t_min {
                    t_min = t0;
                }
                if t1 < t_max {
                    t_max = t1;
                }
            } else {
                if t1 > t_min {
                    t_min = t1;
                }
                if t0 < t_max {
                    t_max = t0;
                }
            }
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> u64 {
        let x = self.x.size();
        let y = self.y.size();
        let z = self.z.size();
        if x > y {
            if x > z { 0 } else { 2 }
        } else {
            if y > z { 1 } else { 2 }
        }
    }
}

impl Add<Vec3> for &Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vec3) -> Aabb {
        Aabb::new(
            &(self.x + rhs.x()),
            &(self.y + rhs.y()),
            &(self.z + rhs.z()),
        )
    }
}

impl Add<&Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, rhs: &Aabb) -> Aabb {
        rhs + self
    }
}
