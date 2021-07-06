pub fn sum_boxes<T: std::ops::Add<Output = T>> (a:Box<T>, b:Box<T>) -> Box<T>{
    Box::new(*a + *b)
}
pub fn compare_boxes<T: PartialEq> (a:Box<T>, b:Box<T>) -> bool{
    *a == *b
}
pub fn greater_than_boxes<T: PartialOrd> (a:Box<T>, b:Box<T>) -> bool{
    *a > *b
}