use crate::material::Material;
use crate::{vec3::Point, ray::Ray, interval::Interval};
use crate::sphere::Sphere;

//
// main trait 
pub trait Hittable {
    
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<(HitRecord, Material)>;

}

pub enum HittableObject {
    Sphere(Sphere),
}

impl Hittable for HittableObject {

    fn hit(&self, ray: &Ray, interval: Interval) -> Option<(HitRecord, Material)> {
        match self {
            Self::Sphere(s) => s.hit(ray, interval),
            // Handle other hittable types here
        }
    }

}

//
// hit record struct
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub hit_location: Point,
    pub normal: Point,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    
    pub fn new(hit_location: Point, normal: Point, t: f32, ray: &Ray) -> Self {

        // normal is assumed to be normalized !!!

        let front_face = ray.direction().dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self { hit_location, normal, t, front_face }

    }

}

//
// hittable list struct
pub struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn new_with_object<T>(object: HittableObject) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }

}

impl Hittable for HittableList {

    fn hit(&self, ray: &Ray, interval: Interval) -> Option<(HitRecord, Material)> {
        
        let mut closest_so_far = interval.max();
        let mut hit_anything = None;

        for object in &self.objects {

            let interval = interval.set_max(closest_so_far);

            if let Some((hit, material)) = object.hit(ray, interval) {

                closest_so_far = hit.t;
                hit_anything = Some((hit, material));
                
            }

        }
        
        return hit_anything;

    }
    
}
