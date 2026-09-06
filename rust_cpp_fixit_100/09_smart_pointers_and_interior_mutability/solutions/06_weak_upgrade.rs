// Topic: Smart Pointers and Interior Mutability
// Solution 06: weak_upgrade
// Fix: upgrade returns Option<Rc<T>>; handle the possibility that the value is gone.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
int main(){auto a=std::make_shared<int>(7);std::weak_ptr<int> w=a;if(auto p=w.lock())std::cout<<*p<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::{Rc,Weak};
fn main(){let a=Rc::new(7);let w:Weak<i32>=Rc::downgrade(&a);println!("{}",*w.upgrade().unwrap());}
