pub struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    pub fn get_area(&self) -> f64{
        self.height * self.width
    }
    pub fn scale(&mut self, scalar:f64){
        self.height *= scalar;
        self.width *= scalar;
    }
    pub fn new(new_width:f64, new_height:f64) -> Rectangle{
        Rectangle{
            width: new_width,
            height: new_height
        }
    }
}
pub struct Circle {
    x:f64,
    y:f64,
    radius:f64
}

impl Circle {
    pub fn get_area(&self) -> f64 {
        3.1415 * self.radius * self.radius
    }
    pub fn get_perimeter(&self) -> f64 {
        2.0 * 3.1415 * self.radius
    }
    pub fn new(x:f64, y:f64, radius:f64) -> Circle {
        Circle {
            x,
            y,
            radius
        }
    }
}