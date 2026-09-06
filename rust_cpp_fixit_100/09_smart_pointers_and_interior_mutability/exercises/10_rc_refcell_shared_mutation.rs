// Topic: Smart Pointers and Interior Mutability
// Exercise 10: rc_refcell_shared_mutation
// Goal: Combine Rc and RefCell intentionally
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <vector>
int main(){auto a=std::make_shared<std::vector<int>>(std::initializer_list<int>{1});auto b=a;b->push_back(2);for(int x:*a)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let a=Rc::new(vec![1]);let b=Rc::clone(&a);b.push(2);println!("{:?}",a);}
