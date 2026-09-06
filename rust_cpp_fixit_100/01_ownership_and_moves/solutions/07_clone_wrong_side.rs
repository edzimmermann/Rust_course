// Topic: Ownership and Moves
// Solution 07: clone_wrong_side
// Fix: Clone at the first assignment so the original remains valid.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string a="A"; auto b=a; auto c=b; std::cout<<a<<" "<<c<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let a=String::from("A"); let b=a.clone(); let c=b.clone(); println!("{a} {c}"); }
