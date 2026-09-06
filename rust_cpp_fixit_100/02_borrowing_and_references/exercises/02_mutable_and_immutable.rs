// Topic: Borrowing and References
// Exercise 02: mutable_and_immutable
// Goal: Mutable vs immutable borrow overlap
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string s="abc"; s.push_back('d'); const std::string& r=s; std::cout<<r<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let mut s=String::from("abc"); let r=&s; s.push('d'); println!("{r}"); }
