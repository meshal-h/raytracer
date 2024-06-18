
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn universe() -> Self {
        Self {min: f32::NEG_INFINITY, max: f32::INFINITY}
    }

    pub fn empty() -> Self {
        Self {min: f32::INFINITY, max: f32::NEG_INFINITY}
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamps (&self, x: f32) -> f32 {
        if x < self.min { return self.min };
        if x > self.max { return self.max };
        return x;
    }

    pub fn set_min(self, min: f32) -> Self {
        Self { min: min, max: self.max }
    }

    pub fn set_max(self, max: f32) -> Self {
        Self { min: self.min, max: max }
    }
}

#[test]
fn test_contains(){
    
    let interval = Interval::universe().set_min(-1.0).set_max(2.0);

    assert_eq!(interval.contains(1.0), true);
    assert_eq!(interval.contains(-0.5), true);
    assert_eq!(interval.contains(-1.0), true);
    assert_eq!(interval.contains(2.0), true);
    assert_eq!(interval.contains(2.5), false);
    assert_eq!(interval.contains(-1.5), false);

}

#[test]
fn test_surrounds(){
    
    let interval = Interval::universe().set_min(-1.0).set_max(2.0);

    assert_eq!(interval.surrounds(1.0), true);
    assert_eq!(interval.surrounds(-0.5), true);
    assert_eq!(interval.surrounds(-1.0), false);
    assert_eq!(interval.surrounds(2.0), false);
    assert_eq!(interval.surrounds(2.5), false);
    assert_eq!(interval.surrounds(-1.5), false);

}

#[test]
fn test_clamps(){
    
    let interval = Interval::universe().set_min(-1.0).set_max(2.0);

    assert_relative_eq!(interval.clamps(1.0), 1.0);
    assert_relative_eq!(interval.clamps(-0.5), -0.5);
    assert_relative_eq!(interval.clamps(-1.0), -1.0);
    assert_relative_eq!(interval.clamps(2.0), 2.0);
    assert_relative_eq!(interval.clamps(2.5), 2.0);
    assert_relative_eq!(interval.clamps(-1.5), -1.0);

}
