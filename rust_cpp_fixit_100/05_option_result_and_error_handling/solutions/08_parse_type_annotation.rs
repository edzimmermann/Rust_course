// Topic: Option Result and Error Handling
// Solution 08: parse_type_annotation
// Fix: Specify the target numeric type for parse.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){int n=std::stoi("42");std::cout<<n+1<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let n:i32="42".parse().unwrap();println!("{}",n+1);}
