// Topic: Strings and Slices
// Solution 05: str_to_string
// Fix: Create an owned String from the string literal.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string greet(){return "hello";}
int main(){std::cout<<greet()<<"\n";}
CPP_EQUIVALENT_END */

fn greet()->String{"hello".to_string()}
fn main(){println!("{}",greet());}
