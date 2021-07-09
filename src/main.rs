mod game;
mod roster;
mod shapes;
mod box_compare;
mod display_trait;
mod location;
fn main() {
    game::bataille_with_stack();
    println!("{}",p(15,10));
}
fn p(a:i32,b:i32)->i32{if a%b==0{return a;}p(b,a%b)}

