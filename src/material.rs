use crate::onb::*;
use crate::rtweekend::*;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn emited(&self, r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
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
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let uvw = Onb::new(rec.normal);
        let scatter_direction = uvw.transform(random_cosine_direction());

        *scattered = Ray::new_with_time(rec.p, unit_vector(scatter_direction), r.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = dot(&uvw.w(), scattered.direction()) / PI;
        true
    }

    fn scattering_pdf(&self, _r: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        1.0 / (2.0 * PI)
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
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let mut reflected = reflect(r.direction(), &rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vec());
        let s = Ray::new_with_time(rec.p, reflected, r.time());
        let a = &self.albedo.value(rec.u, rec.v, &rec.p);
        *scattered = s;
        *attenuation = *a;
        dot(scattered.direction(), &rec.normal) > 0.0
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
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let a = &self
            .albedo
            .value(rec.u, rec.v, &rec.p)
            .powf(self.absorbance);
        *attenuation = *a;

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

        let s = Ray::new_with_time(rec.p, direction, r.time());
        *scattered = s;
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
    fn scatter(
        &self,
        _r: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
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
    fn scatter(
        &self,
        r: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new_with_time(rec.p, random_unit_vec(), r.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *pdf = 1.0 / (4.0 * PI);
        true
    }

    fn scattering_pdf(&self, _r: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
