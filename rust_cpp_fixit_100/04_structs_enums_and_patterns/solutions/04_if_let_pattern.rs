// Topic: Structs Enums and Patterns
// Solution 04: if_let_pattern
// Fix: Use the tuple-variant pattern Some(value).

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
int main(){std::optional<int> n=7;if(n)std::cout<<*n<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let n=Some(7);if let Some(value)=n {println!("{value}");}}
