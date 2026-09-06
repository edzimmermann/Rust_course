// Topic: Borrowing and References
// Solution 01: two_mutable_borrows
// Fix: Make the mutable borrows non-overlapping.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){ int x=10; { int& a=x; ++a; } { int& b=x; ++b; } std::cout<<x<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let mut x=10; { let a=&mut x; *a+=1; } { let b=&mut x; *b+=1; } println!("{x}"); }
