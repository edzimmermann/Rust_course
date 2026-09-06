// Topic: Smart Pointers and Interior Mutability
// Solution 03: refcell_borrow_mut
// Fix: Use borrow_mut to obtain a mutable RefMut guard.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){int x=1;int& r=x;++r;std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::RefCell;
fn main(){let x=RefCell::new(1);let mut r=x.borrow_mut();*r+=1;println!("{}",*r);}
