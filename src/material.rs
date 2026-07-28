use crate::pdf::*;
use crate::rtweekend::*;

pub struct ScatterRecord {
    pub attenuation: Arc<dyn Texture>,
    pub pdf_ptr: Option<Arc<dyn Pdf>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emited(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }
}

pub struct NoMaterial {}

impl NoMaterial {
    pub fn new() -> Self {
        NoMaterial {}
    }
}

impl Material for NoMaterial {}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: impl Into<Arc<dyn Texture>>) -> Lambertian {
        Lambertian {
            albedo: albedo.into(),
        }
    }

    pub fn get_color(&self, r: &HitRecord) -> Color {
        self.albedo.value(r.u, r.v, &r.p)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p).into();
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(&rec.normal)));
        srec.skip_pdf = false;

        true
    }

    fn scattering_pdf(&self, _r: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(*scattered.direction()));
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}

pub struct Metal {
    albedo: Arc<dyn Texture>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: impl Into<Arc<dyn Texture>>, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo.into(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let mut reflected = reflect(r.direction(), &rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vec());

        srec.attenuation = self.albedo.clone();
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new_with_time(rec.p, reflected, r.time());

        true
    }
}

pub struct Dielectric {
    albedo: Arc<dyn Texture>,
    refraction_index: f64,
    absorbance: f64,
}

#[allow(unused)]
impl Dielectric {
    pub fn new(albedo: impl Into<Arc<dyn Texture>>, refraction_index: f64) -> Dielectric {
        Dielectric {
            albedo: albedo.into(),
            refraction_index,
            absorbance: 0.0,
        }
    }

    pub fn with_absorbance(mut self, absorbance: f64) -> Self {
        self.absorbance = absorbance;
        self
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let a: Color = self
            .albedo
            .value(rec.u, rec.v, &rec.p)
            .powf(self.absorbance);

        srec.attenuation = Color::new(a.x, a.y, a.z).into();
        srec.pdf_ptr = None;
        srec.skip_pdf = true;

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(*r.direction());
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (ri * sin_theta) > 1.0;

        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        srec.skip_pdf_ray = Ray::new_with_time(rec.p, direction, r.time());
        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: impl Into<Arc<dyn Texture>>) -> Self {
        DiffuseLight { tex: tex.into() }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }
    fn emited(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: impl Into<Arc<dyn Texture>>) -> Self {
        Isotropic { tex: tex.into() }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p).into();
        srec.pdf_ptr = Some(Arc::new(SpherePdf::new()));
        srec.skip_pdf = false;

        true
    }

    fn scattering_pdf(&self, _r: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
