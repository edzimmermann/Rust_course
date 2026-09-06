// Topic: Borrowing and References
// Solution 02: mutable_and_immutable
// Fix: Perform the mutation before creating the immutable borrow.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string s="abc"; s.push_back('d'); const std::string& r=s; std::cout<<r<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let mut s=String::from("abc"); s.push('d'); let r=&s; println!("{r}"); }
