use crate::vec3::Point;

//
// Ray struct
#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {

    pub fn new(origin: Point, direction: Point) -> Self {
        Self{origin, direction}
    }

    pub fn origin(&self) -> Point {self.origin}

    pub fn direction(&self) -> Point {self.direction}

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction*t
    }

}

#[test]
fn test_ray(){
    let orig = Point::new(1.0, 0.0, -1.0);
    let dir = Point::new(0.5, 1.0, 2.0);
    let ray = Ray::new(orig, dir);

    assert_eq!(ray.at(0.0), orig);
    assert_eq!(ray.at(1.0), Point::new(1.5, 1.0, 1.0));
    assert_eq!(ray.at(-1.0), Point::new(0.5, -1.0, -3.0));
    assert_eq!(ray.origin(), orig);
    assert_eq!(ray.direction(), dir);
}
