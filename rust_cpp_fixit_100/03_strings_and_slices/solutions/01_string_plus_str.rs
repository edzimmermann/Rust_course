// Topic: Strings and Slices
// Solution 01: string_plus_str
// Fix: The + implementation expects &str on the right-hand side.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string a="hello",b=" world";auto c=a+b;std::cout<<c<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let a=String::from("hello");let b=String::from(" world");let c=a+&b;println!("{c}");}
