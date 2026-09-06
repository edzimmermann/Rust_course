// Topic: Smart Pointers and Interior Mutability
// Solution 05: box_deref
// Fix: Dereference the Box to use the contained integer in arithmetic.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
int main(){auto x=std::make_unique<int>(5);int y=*x+1;std::cout<<y<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let x=Box::new(5);let y=*x+1;println!("{y}");}
