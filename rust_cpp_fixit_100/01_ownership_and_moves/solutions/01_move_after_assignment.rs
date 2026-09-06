// Topic: Ownership and Moves
// Solution 01: move_after_assignment
// Fix: Clone the String before assigning it when both values are needed.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){ std::string name="Ada"; std::string other=name; std::cout<<name<<" -> "<<other<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let name=String::from("Ada"); let other=name.clone(); println!("{name} -> {other}"); }
