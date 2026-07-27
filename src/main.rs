mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod rotation;
mod rtw_image;
mod rtweekend;
mod sphere;
mod texture;
mod tri;
mod vec3;

use crate::rtweekend::*;

fn cornell_setup() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    world
}

fn generic_camera() -> Camera {
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;
    cam
}

fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_size = 20;
    for i in 0..boxes_per_size {
        for j in 0..boxes_per_size {
            let i_f = i as f64;
            let j_f = j as f64;
            let w = 100.0;
            let x0 = -1000.0 + i_f * w;
            let z0 = -1000.0 + j_f * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_between(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(generate_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    world.add(Arc::new(BvhNode::new(boxes1)));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 400.0);
    let center2 = center1 + Point3::new(30.0, 0.0, 0.0);
    let sphere_mat = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1, center2, 50.0, sphere_mat,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundry = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5)),
    ));

    world.add(boundry.clone());

    world.add(Arc::new(ConstantMedium::new(
        boundry.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9).into(),
    )));

    boundry = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundry,
        0.0001,
        Color::new(1.0, 1.0, 1.0).into(),
    )));

    let emat = Arc::new(Lambertian::new(ImageTexture::new("earthmap.jpg")));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertex = Arc::new(Lambertian::new(NoiseTexture::new(0.2)));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        pertex,
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::new(
                random_between(0.0, 165.0),
                random_between(0.0, 165.0),
                random_between(0.0, 165.0),
            ),
            10.0,
            white.clone(),
        )));
    }
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(boxes2)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = image_width as u64;
    cam.samples_per_pixel = samples_per_pixel as u64;
    cam.max_depth = max_depth as u64;
    cam.background = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 40.0;
    cam.look_from = Point3::new(478.0, 278.0, -600.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);

    cam.render(&world);
}

fn cornell_obj() {
    let mut world = cornell_setup();

    let mat = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    let obj = Translate::new(
        rotate(
            Arc::new(Scale::new(
                load_obj_triangles("suzanne.obj", mat.clone()),
                Vec3::new(100.0, 100.0, 100.0),
            )),
            Vec3::new(0.0, 180.0, 0.0),
        ),
        Vec3::new(278.0, 278.0, 0.0),
    );
    world.add(Arc::new(obj));

    let mut cam = generic_camera();
    cam.aspect_ratio = 1.0;
    cam.image_width = 200;
    cam.samples_per_pixel = 500;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;

    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.render(&(HittableList::new_from_list(vec![Arc::new(BvhNode::new(world))])));
}

fn cornell_box() {
    let mut world = cornell_setup();

    let white = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
    let box1 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );

    let r_box1 = Arc::new(RotateY::new(box1, 15.0));
    let r_t_box1 = Arc::new(Translate::new(r_box1, Vec3::new(130.0, 0.0, 65.0)));
    world.add(r_t_box1);

    let box2 = generate_box(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    let r_box2 = Arc::new(RotateY::new(box2, -18.0));
    let r_t_box2 = Arc::new(Translate::new(r_box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(r_t_box2);

    let mut cam = generic_camera();
    cam.aspect_ratio = 1.0;
    cam.image_width = 300;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.look_from = Point3::new(278.0, 278.0, -800.0);
    cam.look_at = Point3::new(278.0, 278.0, 0.0);
    cam.render(&world);
}

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Lambertian::new(Checker::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(checker),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // diffuse
                    let color = Color::random() * Color::random();
                    let mat = Arc::new(Lambertian::new(color));
                    let center2 = center + Vec3::new(0.0, random_between(0.0, 0.3), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, mat)));
                } else if choose_mat < 0.85 {
                    //metal
                    let albedo = Color::new(
                        random_between(0.5, 1.0),
                        random_between(0.5, 1.0),
                        random_between(0.5, 1.0),
                    );
                    let mat = Arc::new(Metal::new(albedo, random_between(0.0, 0.5)));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    // glass
                    let mat = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    let material_01 = Arc::new(Metal::new(Color::new(0.99, 0.75, 0.79), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_01,
    )));

    let material_02 = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.50));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_02,
    )));

    let material_04 = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        0.9,
        material_04,
    )));

    let material_03 = Arc::new(Metal::new(NoiseTexture::new(4.0), 0.2));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_03,
    )));
    world = HittableList::new_from_list(vec![Arc::new(BvhNode::new(world))]);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.v_up = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn main() {
    match 7 {
        1 => bouncing_spheres(),
        7 => cornell_box(),
        9 => final_scene(800, 1000, 40),
        11 => cornell_obj(),
        _ => final_scene(400, 250, 4),
    }
}
