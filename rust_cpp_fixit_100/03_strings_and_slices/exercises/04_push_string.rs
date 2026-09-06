// Topic: Strings and Slices
// Exercise 04: push_string
// Goal: push vs push_str
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="hi";s.append("!");std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut s=String::from("hi");s.push("!");println!("{s}");}
