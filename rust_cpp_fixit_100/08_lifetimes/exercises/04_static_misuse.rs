// Topic: Lifetimes
// Exercise 04: static_misuse
// Goal: static means program-long validity
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
std::string_view identity(std::string_view s){return s;}
int main(){std::string s="hi";std::cout<<identity(s)<<"\n";}
CPP_EQUIVALENT_END */

fn as_static(s:&str)->&'static str{s}
fn main(){let s=String::from("hi");println!("{}",as_static(&s));}
