// Topic: Borrowing and References
// Exercise 01: two_mutable_borrows
// Goal: Exclusive mutable borrowing
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){ int x=10; { int& a=x; ++a; } { int& b=x; ++b; } std::cout<<x<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let mut x=10; let a=&mut x; let b=&mut x; *a+=1; *b+=1; println!("{x}"); }
