// Topic: Smart Pointers and Interior Mutability
// Exercise 03: refcell_borrow_mut
// Goal: Interior mutability
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){int x=1;int& r=x;++r;std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::RefCell;
fn main(){let x=RefCell::new(1);let r=x.borrow();*r+=1;println!("{r}");}
