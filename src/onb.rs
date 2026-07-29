use crate::rtweekend::*;

pub struct Onb {
    axis: [Vec3; 3],
}
#[allow(unused)]
impl Onb {
    pub fn new(n: Vec3) -> Self {
        // Frisvad results can be inaccurate so we use Duff's version
        let sign = f64::copysign(1.0, n.z);
        let a = -1.0 / (sign + n.z());
        let b = n.x() * n.y() * a;
        let b1 = Vec3::new(1.0 + sign * n.x() * n.x() * a, sign * b, -sign * n.x());
        let b2 = Vec3::new(b, sign + n.y() * n.y() * a, -n.y());
        Self { axis: [n, b1, b2] }
    }

    pub fn new_a(n: Vec3) -> Self {
        let axis_2 = unit_vector(n);
        let a = if axis_2.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let axis_1 = unit_vector(cross(&axis_2, &a));
        let axis_0 = cross(&axis_2, &axis_1);

        Self {
            axis: [axis_0, axis_1, axis_2],
        }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        (v[0] * self.axis[0]) + (v[1] * self.axis[1]) + (v[2] * self.axis[2])
    }
}
