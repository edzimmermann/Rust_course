// Topic: Ownership and Moves
// Exercise 07: clone_wrong_side
// Goal: Cloning the value that must survive
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string a="A"; auto b=a; auto c=b; std::cout<<a<<" "<<c<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let a=String::from("A"); let b=a; let c=b.clone(); println!("{a} {c}"); }
