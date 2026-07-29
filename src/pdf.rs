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

impl SpherePdf {
    pub fn new() -> Self {
        SpherePdf {}
    }
}

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

pub struct MixturePdf {
    pdf: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        MixturePdf { pdf: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.pdf[0].generate();
        }
        self.pdf[1].generate()
    }

    fn value(&self, direction: &Vec3) -> f64 {
        (0.5 * self.pdf[0].value(direction)) + (0.5 * self.pdf[1].value(direction))
    }
}

pub struct ScatteringPdf {
    amplitude: f64,
    peak_width: f64,
    fall_off: f64,
}

impl ScatteringPdf {
    pub fn new(amplitude: f64, peak_width: f64, fall_off: f64) -> Self {
        Self {
            amplitude,
            peak_width,
            fall_off,
        }
    }
}
