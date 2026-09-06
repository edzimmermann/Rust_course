// Topic: Smart Pointers and Interior Mutability
// Exercise 01: rc_clone
// Goal: Shared ownership with Rc
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto a=std::make_shared<std::string>("data");auto b=a;std::cout<<*a<<" "<<*b<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let a=Rc::new(String::from("data"));let b=a;println!("{} {}",a,b);}
