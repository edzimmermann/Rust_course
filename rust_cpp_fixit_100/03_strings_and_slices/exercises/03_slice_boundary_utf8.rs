// Topic: Strings and Slices
// Exercise 03: slice_boundary_utf8
// Goal: UTF-8 slice boundaries
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="\xC3\xA9" "clair";std::cout<<s.substr(0,2)<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s="éclair";println!("{}",&s[0..1]);}
