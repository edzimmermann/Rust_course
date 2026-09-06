// Topic: Smart Pointers and Interior Mutability
// Exercise 06: weak_upgrade
// Goal: Weak references may be expired
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
int main(){auto a=std::make_shared<int>(7);std::weak_ptr<int> w=a;if(auto p=w.lock())std::cout<<*p<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::{Rc,Weak};
fn main(){let a=Rc::new(7);let w:Weak<i32>=Rc::downgrade(&a);println!("{}",*w.upgrade());}
