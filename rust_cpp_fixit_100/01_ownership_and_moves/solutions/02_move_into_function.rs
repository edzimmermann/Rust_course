// Topic: Ownership and Moves
// Solution 02: move_into_function
// Fix: Borrow the string instead of taking ownership.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
void consume(const std::string& s){ std::cout<<s<<"\n"; }
int main(){ std::string s="hello"; consume(s); std::cout<<s.size()<<"\n"; }
CPP_EQUIVALENT_END */

fn consume(s:&str){ println!("{s}"); }
fn main(){ let s=String::from("hello"); consume(&s); println!("{}",s.len()); }
