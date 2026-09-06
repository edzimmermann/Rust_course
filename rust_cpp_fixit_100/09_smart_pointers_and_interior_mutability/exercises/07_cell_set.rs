// Topic: Smart Pointers and Interior Mutability
// Exercise 07: cell_set
// Goal: Cell uses get and set
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <atomic>
#include <iostream>
int main(){int x=3;x=4;std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::Cell;
fn main(){let x=Cell::new(3);*x=4;println!("{}",x.get());}
