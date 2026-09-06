// Topic: Borrowing and References
// Exercise 09: mutable_binding_needed
// Goal: Binding mutability
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="x"; std::string& r=s; r.push_back('y'); std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let s=String::from("x"); let r=&mut s; r.push('y'); println!("{r}"); }
