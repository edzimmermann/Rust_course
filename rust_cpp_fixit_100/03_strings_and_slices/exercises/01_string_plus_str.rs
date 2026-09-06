// Topic: Strings and Slices
// Exercise 01: string_plus_str
// Goal: String concatenation
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string a="hello",b=" world";auto c=a+b;std::cout<<c<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let a=String::from("hello");let b=String::from(" world");let c=a+b;println!("{c}");}
