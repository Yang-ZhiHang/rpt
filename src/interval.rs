#[derive(Clone, Copy, Default)]
pub struct Interval {
    /// The minimal value of a interval.
    pub min: f32,

    /// The maximal value of a interval.
    pub max: f32,
}

impl Interval {
    /// Create a interval which between `a` and `b`.
    pub fn new(a: f32, b: f32) -> Self {
        let min = if a < b { a } else { b };
        let max = if a > b { a } else { b };
        Self { min, max }
    }

    /// Return the length of the interval.
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    /// Extend both the left and right sides of the interval outward by `delta`.
    pub fn extend(&mut self, delta: f32) {
        self.min = self.min - delta;
        self.max = self.max + delta;
    }

    /// Determine if the interval contains specified value.
    pub fn contains(&self, val: f32) -> bool {
        self.min <= val && val <= self.max
    }

    /// Return the smallest interval that contains both `self` and `other`.
    pub fn union(&self, other: &Interval) -> Interval {
        Interval::new(self.min.min(other.min), self.max.max(other.max))
    }
}
