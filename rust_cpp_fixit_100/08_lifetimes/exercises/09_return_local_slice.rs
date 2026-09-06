// Topic: Lifetimes
// Exercise 09: return_local_slice
// Goal: Slices borrow their source
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string prefix(){std::string s="abcdef";return s.substr(0,3);}
int main(){std::cout<<prefix()<<"\n";}
CPP_EQUIVALENT_END */

fn prefix()->&'static str{let s=String::from("abcdef");&s[..3]}
fn main(){println!("{}",prefix());}
