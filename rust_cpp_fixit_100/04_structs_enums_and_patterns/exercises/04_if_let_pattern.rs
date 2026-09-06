// Topic: Structs Enums and Patterns
// Exercise 04: if_let_pattern
// Goal: Matching Option payload
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
int main(){std::optional<int> n=7;if(n)std::cout<<*n<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let n=Some(7);if let Some = n {println!("seven");}}
