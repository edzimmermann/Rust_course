// Topic: Strings and Slices
// Exercise 05: str_to_string
// Goal: Converting &str to String
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string greet(){return "hello";}
int main(){std::cout<<greet()<<"\n";}
CPP_EQUIVALENT_END */

fn greet()->String{"hello"}
fn main(){println!("{}",greet());}
