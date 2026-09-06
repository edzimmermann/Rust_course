// Topic: Borrowing and References
// Exercise 08: borrowed_temporary
// Goal: Borrowing a temporary
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="temp"; const std::string& r=s; std::cout<<r<<"\n"<<r.size()<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let r; { r=&String::from("temp"); } println!("{r}"); }
