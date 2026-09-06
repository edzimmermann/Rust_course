// Topic: Smart Pointers and Interior Mutability
// Solution 01: rc_clone
// Fix: Clone the Rc handle to increment the reference count.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto a=std::make_shared<std::string>("data");auto b=a;std::cout<<*a<<" "<<*b<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let a=Rc::new(String::from("data"));let b=Rc::clone(&a);println!("{} {}",a,b);}
