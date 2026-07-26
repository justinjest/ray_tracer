use crate::rtweekend::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_depth: u64,
    pub background: Color,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u64,
    pixel_sample_scale: f64,
    sqrt_spp: u64,
    inv_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::new(0.7, 0.8, 1.0), // a kind of light blue
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_sample_scale: 0.0,
            sqrt_spp: 0,
            inv_sqrt_spp: 0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initalize();
        let cam = &*self;

        let width: usize = self.image_width as usize;
        let height: usize = self.image_height as usize;

        let mut image_data = vec![Color::default(); height * width];
        let q: AtomicUsize = AtomicUsize::new(0);
        image_data
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(j, scanline)| {
                eprintln!(
                    "Scanlines remaining {}",
                    height - q.fetch_add(1, Ordering::Relaxed)
                );
                for i in 0..self.image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for s_j in 0..self.sqrt_spp {
                        for s_i in 0..self.sqrt_spp {
                            let r = self.get_ray(i, j as u64, s_j as u64, s_i as u64);
                            pixel_color += self.ray_color(&r, self.max_depth, world);
                        }
                    }
                    scanline[i as usize] = self.pixel_sample_scale * pixel_color;
                }
            });

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for color in &image_data {
            // currently just ignoring error probably not great
            let _ = write_color(&mut io::stdout(), color);
        }
        eprintln!("Done!");
    }

    fn initalize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.sqrt_spp = self.samples_per_pixel.isqrt();
        self.inv_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width) as f64 / self.image_height as f64;

        self.w = unit_vector(self.look_from - self.look_at);
        self.u = unit_vector(cross(&self.v_up, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: u64, j: u64, s_i: u64, s_j: u64) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn sample_square_stratified(&self, s_i: u64, s_j: u64) -> Vec3 {
        let px = ((s_i as f64 * random_double()) * self.inv_sqrt_spp) - 0.5;
        let py = ((s_j as f64 * random_double()) * self.inv_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }

    fn ray_color(&self, r: &Ray, depth: u64, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();

        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let color_from_emission = rec.mat.emited(rec.u, rec.v, &rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let scattering_pdf = rec.mat.scattering_pdf(r, rec, scattered);
        let pdf_value = scattering_pdf;

        let color_from_scatter =
            (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world));

        let color = attenuation * self.ray_color(&scattered, depth - 1, world);

        color_from_emission + color
    }
}
