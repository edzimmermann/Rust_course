// Topic: Borrowing and References
// Exercise 04: mut_parameter_binding
// Goal: Mutable references permit mutation
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
void add_bang(std::string& s){s.push_back('!');}
int main(){std::string s="wow"; add_bang(s); std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn add_bang(s:&String){ s.push('!'); }
fn main(){ let mut s=String::from("wow"); add_bang(&s); println!("{s}"); }
