// Topic: Strings and Slices
// Exercise 02: byte_index_string
// Goal: UTF-8 strings are not indexable
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="Rust";std::cout<<s[0]<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from("Rust");println!("{}",s[0]);}
