// Topic: Traits and Generics
// Solution 03: missing_trait_method
// Fix: Implement the required trait method.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
struct Square{double side;double area()const{return side*side;}};
int main(){std::cout<<Square{2}.area()<<"\n";}
CPP_EQUIVALENT_END */

trait Area{fn area(&self)->f64;}
struct Square{side:f64}
impl Area for Square{fn area(&self)->f64{self.side*self.side}}
fn main(){println!("{}",Square{side:2.0}.area());}
