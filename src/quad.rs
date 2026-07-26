use crate::rtweekend::*;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(n);
        let mut quad = Quad {
            q,
            u,
            v,
            w: n / dot(&n, &n),
            mat,
            bbox: Aabb::empty(),
            normal,
            d: dot(&normal, &q),
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::new_from_points(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::new_from_points(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::new_from_box(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}
impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, r.direction());

        if denom.abs() < 10.0 * (10.0_f64).powf(-18.0) {
            return false;
        }

        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct ImagePlane {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<Lambertian>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl ImagePlane {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<Lambertian>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(n);
        let mut quad = Self {
            q,
            u,
            v,
            w: n / dot(&n, &n),
            mat,
            bbox: Aabb::empty(),
            normal,
            d: dot(&normal, &q),
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diagonal1 = Aabb::new_from_points(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::new_from_points(&(self.q + self.u), &(self.q + self.v));
        self.bbox = Aabb::new_from_box(&bbox_diagonal1, &bbox_diagonal2);
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for ImagePlane {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, r.direction());

        if denom.abs() < 10.0 * (10.0_f64).powf(-18.0) {
            return false;
        }

        let t = (self.d - dot(&self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        if self.mat.get_color(rec) == Color::new(1.0, 1.0, 1.0) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();

        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub fn generate_box(a: &Point3, b: &Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();
    let min = Point3::new(
        f64::min(a.x(), b.x()),
        f64::min(a.y(), b.y()),
        f64::min(a.z(), b.z()),
    );
    let max = Point3::new(
        f64::max(a.x(), b.x()),
        f64::max(a.y(), b.y()),
        f64::max(a.z(), b.z()),
    );

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    ))); // front
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    ))); // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    ))); // back
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    ))); // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    ))); // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    ))); // bottom
    Arc::new(sides)
}
