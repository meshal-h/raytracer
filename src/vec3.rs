use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::default::Default;
use std::fmt::Display;
use std::iter::Sum;

use rand::Rng;
use rand_distr::StandardNormal;

use std::f32::consts::PI;

//
// Vec3 struct
#[derive(Debug, Clone, Copy)] 
pub struct Vec3(f32, f32, f32);

impl Vec3{

    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Self(a, b, c)
    }

    pub fn unit_vector(vec: &Self) -> Self {
        let l = vec.length();
        Self::new(vec.0, vec.1, vec.2) / l
    }

    pub fn length(&self) -> f32 {
        self.length_square().sqrt()
    }

    pub fn length_square(&self) -> f32 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        self.0 /= l;
        self.1 /= l;
        self.2 /= l;
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.0*other.0 + self.1*other.1 + self.2*other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1*other.2 - self.2*other.1,
            self.2*other.0 - self.0*other.2,
            self.0*other.1 - self.1*other.0
        )
    }

}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    } 
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
        )
    } 
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(
            self.0 - rhs,
            self.1 - rhs,
            self.2 - rhs,
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
        )
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
        )
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(
            -self.0,
            -self.1,
            -self.2,
        )
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.0, other.0) && relative_eq!(self.1, other.1) && relative_eq!(self.2, other.2)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

//
// Point type alias
pub type Point = Vec3;

impl Point {
    
    pub fn x(&self) -> f32 {self.0}
    pub fn y(&self) -> f32 {self.1}
    pub fn z(&self) -> f32 {self.2}

    pub fn random_float() -> f32 {

        let mut rng = rand::thread_rng();
        let x = rng.gen();

        return x;

    }

    pub fn random_vec() -> Self {

        let mut rng = rand::thread_rng();
        let x = rng.gen();
        let y = rng.gen();
        let z = rng.gen();
        let vec = Self::new(x, y, z);

        return vec;

    }

    pub fn random_on_sphere() -> Self {
        
        let mut rng = rand::thread_rng();
        let x = rng.sample::<f32,_>(StandardNormal);
        let y = rng.sample::<f32,_>(StandardNormal);
        let z = rng.sample::<f32,_>(StandardNormal);

        let mut vec = Self::new(x, y, z);
        vec.normalize();

        return vec;

    }

    pub fn random_in_unit_disk() -> Self {

        let mut rng = rand::thread_rng();

        let r = rng.gen_range::<f32,_>(0.0..1.0).sqrt();
        let theta = rng.gen_range::<f32,_>(0.0..2.0*PI);

        let x = r * theta.cos();
        let y = r * theta.sin();
        let z = 0.0;

        let vec = Self::new(x, y, z);

        return vec;

    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {

        let mut rand_vec = Self::random_on_sphere();

        if normal.dot(rand_vec) < 0.0 { rand_vec = -rand_vec }

        return rand_vec;

    }

    pub fn reflect(ray: Self, normal: Self) -> Self {
        ray - normal*ray.dot(normal)*2.0
    }

    pub fn refract(ray_in: Self, normal: Self, eta_frac: f32) -> Self {

        let cos_theta = (normal.dot(-ray_in)).min(1.0);
        let ray_out_perpendicular = ( ray_in + normal*cos_theta )*eta_frac;
        let ray_out_parallel = -normal * (1.0 - ray_out_perpendicular.length_square()).abs().sqrt();

        return ray_out_perpendicular + ray_out_parallel;

    }

}

//
// Color type alias
pub type Color = Vec3;

impl Color{

    // gamma 2 color space
    pub fn r(&self) -> f32 {self.0.min(0.999).sqrt().max(0.0)}
    pub fn g(&self) -> f32 {self.1.min(0.999).sqrt().max(0.0)}
    pub fn b(&self) -> f32 {self.2.min(0.999).sqrt().max(0.0)}

    pub fn as_bytes(&self) -> Vec<u8> {

        let r = (255.999*self.r()) as u8;
        let g = (255.999*self.g()) as u8;
        let b = (255.999*self.b()) as u8;

        format!("{} {} {}\n", r, g, b).into()
    }

}

//
// tests
#[test]
fn test_add(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(1.0, -1.0, 1.0);

    assert_eq!(a.clone() + 0.5, Vec3::new(1.5, 1.0, 0.5));
    assert_eq!(a + b, Vec3::new(2.0, -0.5, 1.0));
}

#[test]
fn test_sub(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(1.0, -1.0, 1.0);

    assert_eq!(a.clone() - 0.5, Vec3::new(0.5, 0.0, -0.5));
    assert_eq!(a - b, Vec3::new(0.0, 1.5, -1.0));
}

#[test]
fn test_mul(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(1.0, -1.0, 1.0);

    assert_eq!(a.clone() * 0.5, Vec3::new(0.5, 0.25, 0.0));
    assert_eq!(a * b, Vec3::new(1.0, -0.5, 0.0));
}

#[test]
fn test_div(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(2.0, -1.0, 1.0);

    assert_eq!(a.clone() / 0.5, Vec3::new(2.0, 1.0, 0.0));
    assert_eq!(a / b, Vec3::new(0.5, -0.5, 0.0));
}

#[test]
fn test_neg(){
    let a = Vec3::new(1.0, -0.5, 0.0);

    assert_eq!(-a, Vec3::new(-1.0, 0.5, 0.0));
}

#[test]
fn test_length(){
    let a = Vec3::new(1.0, -0.5, 0.0);

    assert_relative_eq!(a.length_square(), 1.25);
    assert_relative_eq!(a.length(), f32::sqrt(1.25));
}

#[test]
fn test_unit_vector(){
    let a = Vec3::new(1.0, -0.5, 0.0);
    let a = Vec3::unit_vector(&a);
    let mut b = Vec3::new(1.0, -0.5, 0.0);
    b.normalize();

    assert_relative_eq!(a.length(), 1.0);
    assert_relative_eq!(b.length(), 1.0);
}

#[test]
fn test_dot(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(2.0, -1.0, 1.0);

    assert_relative_eq!(a.dot(b), 1.5);
}

#[test]
fn test_cross(){
    let a = Vec3::new(1.0, 0.5, 0.0);
    let b = Vec3::new(2.0, -1.0, 1.0);

    assert_eq!(a.cross(b), Vec3::new(0.5, -1.0, -2.0));
}

#[test]
fn test_display(){
    let a = Vec3::new(1.0, 0.5, 0.0);

    assert_eq!(a.to_string(), format!("{} {} {}", 1.0, 0.5, 0.0));
}

#[test]
fn test_color(){
    let a = Color::new(2.0, 0.5, -1.0);

    assert_eq!(a.as_bytes(), "255 181 0\n".as_bytes());
    assert_relative_eq!(a.r(), f32::sqrt(0.999));
    assert_relative_eq!(a.g(), f32::sqrt(0.5));
    assert_relative_eq!(a.b(), 0.0);
}

#[test]
fn test_point(){
    let a = Point::new(1.75, 2.5, -3.0);

    assert_relative_eq!(a.x(), 1.75);
    assert_relative_eq!(a.y(), 2.5);
    assert_relative_eq!(a.z(), -3.0);
}
