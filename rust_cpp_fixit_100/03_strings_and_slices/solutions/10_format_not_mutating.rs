// Topic: Strings and Slices
// Solution 10: format_not_mutating
// Fix: Bind the String produced by format!.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <sstream>
#include <string>
int main(){std::ostringstream out;out<<"value="<<42;std::string s=out.str();std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from("value=");let s=format!("{s}{}",42);println!("{s}");}
