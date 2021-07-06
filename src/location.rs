use std::fmt;

pub enum Location{
    Unknown,
    Anonymous,
    Known(f64,f64)
}

impl Location {
    pub fn display (&self) {
        match *self {
          Location::Unknown => println!("the location is unknown"),
            Location::Anonymous => println!("the location doesn't want to be shown"),
            Location::Known(n,e) =>  println!("Coordiantes are {} N, {} E", n, e),
        }
    }
}
impl fmt::Display for Location {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Location::Unknown => write!(f, "the location is unknown"),
            Location::Anonymous => write!(f, "the location doesn't want to be shown"),
            Location::Known(n, e) => write!(f, "Coordiantes are {} N, {} E", n, e),
        }
    }
}