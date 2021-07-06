use std::fmt;
use std::cmp::Ordering;


pub struct Satellite {
    pub name: String,
    pub velocity: i32 // miles per second
}

impl fmt::Display for Satellite {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Satellite name is {}, its velocity is {}", self.name, self.velocity)
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.velocity == other.velocity
    }
}

impl Ord for Satellite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.velocity.cmp(&other.velocity)
    }
}

impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Satellite{}