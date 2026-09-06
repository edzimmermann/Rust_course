// Topic: Smart Pointers and Interior Mutability
// Solution 07: cell_set
// Fix: Use Cell::set rather than dereference assignment.

/* CPP_EQUIVALENT_BEGIN
#include <atomic>
#include <iostream>
int main(){int x=3;x=4;std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

use std::cell::Cell;
fn main(){let x=Cell::new(3);x.set(4);println!("{}",x.get());}
