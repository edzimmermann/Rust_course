// Topic: Ownership and Moves
// Exercise 02: move_into_function
// Goal: Passing ownership to functions
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
void consume(const std::string& s){ std::cout<<s<<"\n"; }
int main(){ std::string s="hello"; consume(s); std::cout<<s.size()<<"\n"; }
CPP_EQUIVALENT_END */

fn consume(s:String){ println!("{s}"); }
fn main(){ let s=String::from("hello"); consume(s); println!("{}",s.len()); }
