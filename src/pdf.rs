use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::*;

pub trait Pdf: Send + Sync {
    fn value(&self, _direction: &Vec3) -> f64 {
        0.0
    }

    fn generate(&self) -> Vec3 {
        Vec3::empty()
    }
}

pub struct SpherePdf {}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 / PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vec()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(dir: &Vec3) -> Self {
        Self {
            uvw: Onb::new(*dir),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(*direction), &self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}

pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
