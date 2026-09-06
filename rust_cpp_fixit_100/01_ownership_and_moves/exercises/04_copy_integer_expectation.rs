// Topic: Ownership and Moves
// Exercise 04: copy_integer_expectation
// Goal: Copy types
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){ int n=7; int m=n; std::cout<<n<<" "<<m<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let n=7; let m=n; drop(n); println!("{n} {m}"); }
