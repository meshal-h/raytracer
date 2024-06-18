use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::{Color, Point};

pub fn final_scene() -> (HittableList, Camera) {

    // world
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -11..11 {
        for b in -11..11 {

            let choose_material = Point::random_float();
            let center = Point::new(a as f32 + 0.9*Point::random_float(), 0.2, b as f32 + 0.9*Point::random_float());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_material < 0.8 {

                    let albedo = Color::random_vec() * Color::random_vec();
                    let material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, material));
                    
                } else if choose_material < 0.95 {

                    let albedo = Color::random_vec()*0.5 + 0.5;
                    let fuzz = Point::random_float()*0.5;
                    let material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, material));

                } else {
                    
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));

                }
            }

        }
    }

    let material = Dielectric::new(1.5);
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material));

    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material));

    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material));

    // camera
    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let v_fov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let v_up = Point::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth,
         v_fov, look_from, look_at, v_up, defocus_angle, focus_distance);

    return (world, camera);

}

pub fn penultimate_scene() -> (HittableList, Camera) {

    // world
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.1, 0.6, 0.1));
    let material_center = Lambertian::new(Color::new(0.6, 0.0, 0.0));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.0 / 1.50);
    let material_right = Metal::new(Color::new(0.05, 0.05, 0.80), 0.2);

    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right));

    // camera
    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let v_fov = 20.0;
    let look_from = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let v_up = Point::new(0.0, 1.0, 0.0);

    let defocus_angle = 2.0;
    let focus_distance = 3.4;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth,
         v_fov, look_from, look_at, v_up, defocus_angle, focus_distance);

    return (world, camera);

}
