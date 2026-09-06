// Topic: Lifetimes
// Solution 04: static_misuse
// Fix: Do not promise a static lifetime for data borrowed from the caller.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
std::string_view identity(std::string_view s){return s;}
int main(){std::string s="hi";std::cout<<identity(s)<<"\n";}
CPP_EQUIVALENT_END */

fn identity(s:&str)->&str{s}
fn main(){let s=String::from("hi");println!("{}",identity(&s));}
