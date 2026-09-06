// Topic: Strings and Slices
// Solution 04: push_string
// Fix: Use push_str for a string slice; push takes one char.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="hi";s.append("!");std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut s=String::from("hi");s.push_str("!");println!("{s}");}
