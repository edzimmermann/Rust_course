// Topic: Traits and Generics
// Exercise 03: missing_trait_method
// Goal: Trait implementation completeness
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Square{double side;double area()const{return side*side;}};
int main(){std::cout<<Square{2}.area()<<"\n";}
CPP_EQUIVALENT_END */

trait Area{fn area(&self)->f64;}
struct Square{side:f64}
impl Area for Square{}
fn main(){println!("{}",Square{side:2.0}.area());}
