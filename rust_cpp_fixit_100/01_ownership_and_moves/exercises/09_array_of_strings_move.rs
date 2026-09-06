// Topic: Ownership and Moves
// Exercise 09: array_of_strings_move
// Goal: Indexing does not move owned String
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <array>
#include <iostream>
#include <string>
int main(){ std::array<std::string,2> a{"x","y"}; const auto& first=a[0]; std::cout<<first<<" "<<a[1]<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let a=[String::from("x"),String::from("y")]; let first=a[0]; println!("{first} {}",a[1]); }
