// Topic: Traits and Generics
// Exercise 09: from_conversion
// Goal: From and Into conversions
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s=std::to_string(42);std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from(42);println!("{s}");}
