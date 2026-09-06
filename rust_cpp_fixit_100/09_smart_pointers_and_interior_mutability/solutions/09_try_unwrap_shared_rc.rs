// Topic: Smart Pointers and Interior Mutability
// Solution 09: try_unwrap_shared_rc
// Fix: Drop other strong Rc handles before try_unwrap.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto a=std::make_shared<std::string>("x");{auto b=a;}std::cout<<*a<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let a=Rc::new(String::from("x"));let b=Rc::clone(&a);drop(b);println!("{}",Rc::try_unwrap(a).unwrap());}
