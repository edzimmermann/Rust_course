// Topic: Ownership and Moves
// Exercise 01: move_after_assignment
// Goal: Moves vs copies
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string name="Ada"; std::string other=name; std::cout<<name<<" -> "<<other<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let name=String::from("Ada"); let other=name; println!("{name} -> {other}"); }
