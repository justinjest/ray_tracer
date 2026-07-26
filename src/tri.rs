use crate::rtweekend::*;
use std::path::Path;
use tobj::{LoadOptions, GPU_LOAD_OPTIONS};

pub struct Tri {
    p0: Point3,
    p1: Point3,
    p2: Point3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Tri {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: Arc<dyn Material>) -> Self {
        let mut tri = Tri {
            p0,
            p1,
            p2,
            mat,
            bbox: Aabb::empty(),
        };

        tri.set_bounding_box();
        tri
    }

    fn set_bounding_box(&mut self) {
        let min_x = self.p0.x.min(self.p1.x).min(self.p2.x);
        let min_y = self.p0.y.min(self.p1.y).min(self.p2.y);
        let min_z = self.p0.z.min(self.p1.z).min(self.p2.z);

        let max_x = self.p0.x.max(self.p1.x).max(self.p2.x);
        let max_y = self.p0.y.max(self.p1.y).max(self.p2.y);
        let max_z = self.p0.z.max(self.p1.z).max(self.p2.z);
        self.bbox = Aabb::new_from_points(
            &Point3::new(min_x, min_y, min_z),
            &Point3::new(max_x, max_y, max_z),
        );
    }
}

impl Hittable for Tri {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let epsillon = 1e-8;

        let edge1: Vec3 = self.p1 - self.p0;
        let edge2: Vec3 = self.p2 - self.p0;
        let h = cross(r.direction(), &edge2);
        let a = dot(&edge1, &h);
        if a < epsillon && a > -epsillon {
            return false;
        }

        let f = 1.0 / a;
        let s = *r.origin() - self.p0;
        let u = f * dot(&s, &h);
        if !(0.0..=1.0).contains(&u) {
            return false;
        }
        let q = cross(&s, &edge1);
        let v = f * dot(r.direction(), &q);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }
        let t = f * dot(&edge2, &q);
        if !ray_t.contains(t) {
            return false;
        }
        rec.t = t;
        rec.p = r.at(rec.t);
        rec.mat = self.mat.clone();
        rec.u = u;
        rec.v = v;
        rec.set_face_normal(r, unit_vector(cross(&edge1, &edge2)));
        true
    }

    fn bounding_box(&self) -> Aabb {
        // self.bbox
        Aabb::new_from_points(
            &Point3::new(-100.0, -100.0, -100.0),
            &Point3::new(100.0, 100.0, 100.0),
        )
    }
}

fn load_obj_from_path(filename: &str) -> Option<(Vec<tobj::Model>, Vec<tobj::Material>)> {
    let search_paths = [
        filename.to_string(),
        format!("objects/{}", filename).to_string(),
        format!("../objects/{}", filename).to_string(),
        format!("../../objects/{}", filename).to_string(),
    ];

    for path in &search_paths {
        if Path::new(path).exists() {
            let (models, materials) = tobj::load_obj(
                path,
                &LoadOptions {
                    triangulate: true, // Automatically converts quads into triangles
                    single_index: true,
                    ..GPU_LOAD_OPTIONS
                },
            )
            .expect("Failed to load OBJ file");

            return Some((models, materials.expect("Fialed to process materials")));
        }
    }
    None
}

pub fn load_obj_triangles(filename: &str, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    // Load the OBJ file (triangulating non-triangle faces automatically)
    let (models, _) = load_obj_from_path(filename).expect("Unable to load obj file");

    let mut triangles = HittableList::new();

    let mut min_x = INFINITY;
    let mut min_y = INFINITY;
    let mut min_z = INFINITY;

    let mut max_x = -INFINITY;
    let mut max_y = -INFINITY;
    let mut max_z = -INFINITY;

    for model in models {
        let mesh = &model.mesh;

        // Iterate through indices 3 at a time (each group of 3 forms 1 triangle)
        for chunk in mesh.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            // Extract x, y, z positions for each vertex index
            let p0 = Point3::new(
                mesh.positions[3 * i0] as f64,
                mesh.positions[3 * i0 + 1] as f64,
                mesh.positions[3 * i0 + 2] as f64,
            );

            let p1 = Point3::new(
                mesh.positions[3 * i1] as f64,
                mesh.positions[3 * i1 + 1] as f64,
                mesh.positions[3 * i1 + 2] as f64,
            );

            let p2 = Point3::new(
                mesh.positions[3 * i2] as f64,
                mesh.positions[3 * i2 + 1] as f64,
                mesh.positions[3 * i2 + 2] as f64,
            );

            min_x = min_x.min(p0.x()).min(p1.x()).min(p2.x());
            min_y = min_y.min(p0.y()).min(p1.y()).min(p2.y());
            min_z = min_z.min(p0.z()).min(p1.z()).min(p2.z());

            max_x = max_x.max(p0.x()).max(p1.x()).max(p2.x());
            max_y = max_y.max(p0.y()).max(p1.y()).max(p2.y());
            max_z = max_z.max(p0.z()).max(p1.z()).max(p2.z());

            let center_vec = -Vec3::new(
                (min_x + max_x) / 2.0,
                (min_y + max_y) / 2.0,
                (min_z + max_z) / 2.0,
            );

            // Construct your ray tracer's Triangle
            triangles.add(Arc::new(Translate::new(
                Arc::new(Tri::new(p0, p1, p2, mat.clone())),
                center_vec,
            )));
        }
    }

    Arc::new(BvhNode::new(triangles))
}
