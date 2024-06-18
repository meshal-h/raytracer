use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, Point}};

use rand_distr::num_traits::pow;

//
// main trait
pub trait Scatter {
    
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;

}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatter for Material {
    
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian(l) => l.scatter(record),
            Self::Metal(m) => m.scatter(ray_in, record),
            Self::Dielectric(d) => d.scatter(ray_in, record),
            // Handle other materials here
        }
    }

}

//
// Lambertian (diffuse)
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {

    albedo: Color,

}

impl Lambertian {

    pub fn new(albedo: Color) -> Material {
        Material::Lambertian(Self { albedo })
    }

    fn scatter(&self, record: &HitRecord) -> Option<(Ray, Color)> {

        let mut scatter_direction = record.normal + Point::random_on_sphere();

        if scatter_direction == Point::new(0.0, 0.0, 0.0) { scatter_direction = record.normal }

        let ray_out = Ray::new(record.hit_location, scatter_direction);

        let attenuation = self.albedo;

        return Some((ray_out, attenuation));

    }

}

//
// Metal
#[derive(Debug, Clone, Copy)]
pub struct Metal {

    albedo: Color,
    fuzz: f32,

}

impl Metal {

    pub fn new(albedo: Color, fuzz: f32) -> Material {
        Material::Metal(Self { albedo, fuzz })
    }

    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {

        let reflected = Point::reflect(ray_in.direction(), record.normal);
        let reflected = Point::unit_vector(&reflected) + Point::random_on_sphere()*self.fuzz;

        if reflected.dot(record.normal) < 0.0 { return None }

        let ray_out = Ray::new(record.hit_location, reflected);

        let attenuation = self.albedo;

        return Some((ray_out, attenuation));

    }

}

//
// Dielectric
#[derive(Debug, Clone, Copy)]
pub struct Dielectric {

    refraction_index: f32,

}

impl Dielectric {

    pub fn new(refraction_index: f32) -> Material {
        Material::Dielectric(Self { refraction_index })
    }

    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {

        let eta_frac = if record.front_face { 1.0/self.refraction_index } else { self.refraction_index };
        let ray_in_normalized = Point::unit_vector(&ray_in.direction());

        let cos_theta = (record.normal.dot(-ray_in_normalized)).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction: Point;
        let cannot_refract = eta_frac * sin_theta > 1.0;

        if cannot_refract || Dielectric::reflectance(cos_theta, eta_frac) > Point::random_float() {
            direction = Point::reflect(ray_in_normalized, record.normal);
        } else {
            direction = Point::refract(ray_in_normalized, record.normal, eta_frac);
        }

        let ray_out = Ray::new(record.hit_location, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        return Some((ray_out, attenuation));

    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {

        // Schlick's approximation for reflectance
        let r = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r = r*r;
        let r = r + (1.0-r)*pow(1.0-cosine, 5);

        return r;

    }

}
