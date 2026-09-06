// Topic: Smart Pointers and Interior Mutability
// Exercise 05: box_deref
// Goal: Dereferencing Box values
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
int main(){auto x=std::make_unique<int>(5);int y=*x+1;std::cout<<y<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let x=Box::new(5);let y=x+1;println!("{y}");}
