// Topic: Smart Pointers and Interior Mutability
// Exercise 09: try_unwrap_shared_rc
// Goal: Unwrapping Rc requires unique ownership
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto a=std::make_shared<std::string>("x");{auto b=a;}std::cout<<*a<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let a=Rc::new(String::from("x"));let _b=Rc::clone(&a);println!("{}",Rc::try_unwrap(a).unwrap());}
