// Topic: Smart Pointers and Interior Mutability
// Exercise 04: refcell_runtime_borrow
// Goal: RefCell borrow rules are runtime checked
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> x{1};for(int n:x)std::cout<<n<<" ";x.push_back(2);std::cout<<x.back()<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::RefCell;
fn main(){let x=RefCell::new(vec![1]);let r=x.borrow();x.borrow_mut().push(2);println!("{:?}",*r);}
