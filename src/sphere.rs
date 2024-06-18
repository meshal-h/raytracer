use crate::{vec3::Point, ray::Ray, interval::Interval};
use crate::hittable::{Hittable, HitRecord, HittableObject};
use crate::material::Material;

pub struct Sphere {
    center: Point,
    radius: f32,
    material: Material,
}

impl Sphere {
    
    pub fn new(center: Point, radius: f32, material: Material) -> HittableObject {
        HittableObject::Sphere(Self { center, radius, material })
    }
    
}

impl Hittable for Sphere {
    
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<(HitRecord, Material)> {
        
        // hit logic
        let oc = self.center - ray.origin();

        let a = ray.direction().length_square();
        let h = ray.direction().dot(oc);
        let c = oc.length_square() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 { return  None }

        // find the nearest hit in [t_min:t_max]
        let dis_sqrt = discriminant.sqrt();
        let mut root = (h - dis_sqrt) / a;

        if ! interval.surrounds(root) {
            
            root = (h + dis_sqrt) / a;

            if ! interval.surrounds(root) { return None }

        }

        // generate record
        let hit_location = ray.at(root);
        let normal = (hit_location - self.center) / self.radius; // outward normal
        let record = HitRecord::new(hit_location, normal, root, ray);

        return Some((record, self.material));

    }

}

//
// tests
#[test]
fn test_hit(){
    
    use crate::interval::Interval;

    let radius = 1.0;
    let center = Point::new(0.0, 1.0, 0.0);
    let material = crate::material::Lambertian::new(Point::new(0.5, 0.5, 0.5));
    let sphere = Sphere::new(center, radius, material);

    let ray_origin = Point::new(-2.0, 1.0, 0.0);
    let ray_direction = Point::new(1.0, 0.0, 0.0);
    let ray = Ray::new(ray_origin, ray_direction);

    let interval = Interval::universe().set_min(0.0);
    let (record, _) = sphere.hit(&ray, interval).expect("There should be a hit.");
    assert_eq!(record.hit_location, Point::new(-1.0, 1.0, 0.0));
    assert_relative_eq!(record.t, 1.0);

    let ray_origin = Point::new(-1.0, 1.0, 0.0);
    let ray = Ray::new(ray_origin, ray_direction);

    let interval = Interval::universe().set_min(0.0001);
    let (record, _) = sphere.hit(&ray, interval).expect("There should be a hit.");
    assert_eq!(record.hit_location, Point::new(1.0, 1.0, 0.0));
    assert_relative_eq!(record.t, 2.0);

}
