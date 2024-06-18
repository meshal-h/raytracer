use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
// use indicatif::ProgressIterator;
use rand::Rng;

use std::f32::consts::PI;

use crate::hittable::{Hittable, HittableList};
use crate::material::Scatter;
use crate::vec3::{Point, Color};
use crate::interval::Interval;
use crate::ray::Ray;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub struct Camera {

    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    pub v_fov: f32,
    pub look_from: Point,
    pub look_at: Point,
    pub v_up: Point,

    pub defocus_angle: f32,
    pub focus_distance: f32,

    image_height: i32,
    center: Point,
    pixel_00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
    pixel_samples_scale: f32,
    u: Point,
    v: Point,
    w: Point,
    defocus_disk_u: Point,
    defocus_disk_v: Point,

}

impl Camera {
    
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32, max_depth: i32, 
        v_fov: f32, look_from: Point, look_at: Point, v_up: Point, defocus_angle: f32, focus_distance: f32) -> Self {

        let mut camera = Self::default();

        // initialize camera
        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.samples_per_pixel = samples_per_pixel;
        camera.max_depth = max_depth;
        camera.v_fov = v_fov;
        camera.look_from = look_from;
        camera.look_at = look_at;
        camera.v_up = v_up;
        camera.defocus_angle = defocus_angle;
        camera.focus_distance = focus_distance;
        camera.initialize();

        return camera;
        
    }

    pub fn render(&self, world: &HittableList) -> Vec<u8> {
        
        // initialize buffer
        let mut buffer: Vec<u8> = Vec::new();

        // PPM header
        let header_buffer = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        buffer.extend(header_buffer.as_bytes());

        // image
        let image_buffer = (0..self.image_height)
        .into_par_iter()
        .progress_count(self.image_height as u64)
        .flat_map(|j| {

            let row_buffer = (0..self.image_width)
                .into_par_iter()
                .flat_map(|i| {

                    let pixel_color: Color = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {

                        let ray = self.get_ray(i, j);
                        let color = self.ray_color(&ray, self.max_depth, &world);

                        return color;

                    }).sum();
                    
                    let average_pixel_color = pixel_color * self.pixel_samples_scale;
                    
                    return average_pixel_color.as_bytes();

                })
                .collect::<Vec<u8>>();
            
            return row_buffer;
                
        })
        .collect::<Vec<u8>>();

        buffer.extend(image_buffer);

        return buffer;
        
    }

    fn initialize(&mut self) {

        // camera param
        self.image_height = ( (self.image_width as f32) / self.aspect_ratio ) as i32;
        self.image_height = self.image_height.max(1);

        self.center = self.look_from;

        // sampling param
        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f32);

        // viewport param
        let theta = degrees_to_radians(self.v_fov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * ( self.image_width as f32) / (self.image_height as f32 );

        // basis vectors
        self.w = Point::unit_vector(&(self.look_from - self.look_at));
        self.u = Point::unit_vector(&(self.v_up.cross(self.w)));
        self.v = self.w.cross(self.u);

        // viewport vectors
        let viewport_u = self.u*viewport_width;
        let viewport_v = -self.v*viewport_height;
        self.pixel_delta_u = viewport_u / (self.image_width as f32);
        self.pixel_delta_v = viewport_v / (self.image_height as f32);

        // reference points
        let viewport_upper_left = self.center - self.w*self.focus_distance - viewport_u/2.0 - viewport_v/2.0;
        self.pixel_00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v)*0.5;

        // camera defocus
        let defocus_radius = self.focus_distance * (degrees_to_radians(self.defocus_angle/2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {

        // anti-aliasing
        let offset = Camera::sample_square();

        let x_offset = self.pixel_delta_u * ( (i as f32) + offset.x() );
        let y_offset = self.pixel_delta_v * ( (j as f32) + offset.y() );

        let pixel_sample = self.pixel_00_loc + x_offset + y_offset;

        // generate ray
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);

    }
    
    fn sample_square() -> Point {

        // rand in [-0.5:0.5]
        let mut rng = rand::thread_rng();
        let rand_x  = rng.gen::<f32>() - 0.5;
        let rand_y = rng.gen::<f32>() - 0.5;
        
        return Point::new(rand_x, rand_y, 0.0);
        
    }

    fn defocus_disk_sample(&self) -> Point {

        let p = Point::random_in_unit_disk();
        let sample = self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y();

        return sample;

    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &HittableList) -> Color {

        // stop gathering light if depth is exceeded
        if depth <= 0 { return Color::default() }

        let interval = Interval::universe().set_min(0.001);
    
        if let Some((hit, material)) = world.hit(ray, interval) {

            if let Some((ray_out, attenuation)) = material.scatter(ray, &hit) {

                return attenuation*self.ray_color(&ray_out, depth-1, world);

            } else { return Color::default() }

        }
    
        let unit_direction = Point::unit_vector(&ray.direction());
        let a = 0.5*(unit_direction.y() + 1.0);
    
        return Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a;
    
    }

}

impl Default for Camera {
    
    fn default() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            samples_per_pixel: 0,
            max_depth: 0,
            v_fov: 0.0,
            look_from: Point::default(),
            look_at: Point::default(),
            v_up: Point::default(),
            defocus_angle: 0.0,
            focus_distance: 0.0,
            image_height: 0,
            center: Point::default(),
            pixel_00_loc: Point::default(),
            pixel_delta_u: Point::default(),
            pixel_delta_v: Point::default(),
            pixel_samples_scale: 0.0,
            u: Point::default(),
            v: Point::default(),
            w: Point::default(),
            defocus_disk_u: Point::default(),
            defocus_disk_v: Point::default(),
        }
    }

}
