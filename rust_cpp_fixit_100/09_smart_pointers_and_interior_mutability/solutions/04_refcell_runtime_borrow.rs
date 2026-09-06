// Topic: Smart Pointers and Interior Mutability
// Solution 04: refcell_runtime_borrow
// Fix: End the immutable RefCell borrow before taking a mutable borrow.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> x{1};for(int n:x)std::cout<<n<<" ";x.push_back(2);std::cout<<x.back()<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::RefCell;
fn main(){let x=RefCell::new(vec![1]);{let r=x.borrow();println!("{:?}",*r);}x.borrow_mut().push(2);println!("{:?}",x.borrow());}
