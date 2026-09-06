// Topic: Traits and Generics
// Solution 09: from_conversion
// Fix: Use ToString/Display formatting; String does not implement From<i32>.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s=std::to_string(42);std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=42.to_string();println!("{s}");}
