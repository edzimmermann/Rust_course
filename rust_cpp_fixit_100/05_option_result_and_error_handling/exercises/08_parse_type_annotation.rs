// Topic: Option Result and Error Handling
// Exercise 08: parse_type_annotation
// Goal: Type inference for parse
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){int n=std::stoi("42");std::cout<<n+1<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let n="42".parse().unwrap();println!("{}",n+1);}
