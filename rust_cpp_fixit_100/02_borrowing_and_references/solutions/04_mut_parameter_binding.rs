// Topic: Borrowing and References
// Solution 04: mut_parameter_binding
// Fix: Use &mut String in the function and at the call site.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
void add_bang(std::string& s){s.push_back('!');}
int main(){std::string s="wow"; add_bang(s); std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn add_bang(s:&mut String){ s.push('!'); }
fn main(){ let mut s=String::from("wow"); add_bang(&mut s); println!("{s}"); }
