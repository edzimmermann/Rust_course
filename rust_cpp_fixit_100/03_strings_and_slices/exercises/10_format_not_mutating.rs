// Topic: Strings and Slices
// Exercise 10: format_not_mutating
// Goal: format! creates a String
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <sstream>
#include <string>
int main(){std::ostringstream out;out<<"value="<<42;std::string s=out.str();std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut s=String::from("value=");format!("{s}{}",42);println!("{s}");}
