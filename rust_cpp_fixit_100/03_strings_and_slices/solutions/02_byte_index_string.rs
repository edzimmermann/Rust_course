// Topic: Strings and Slices
// Solution 02: byte_index_string
// Fix: Use chars() (or bytes()) rather than indexing a Rust String.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="Rust";std::cout<<s[0]<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from("Rust");println!("{}",s.chars().next().unwrap());}
