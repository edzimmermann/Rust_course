// Topic: Ownership and Moves
// Exercise 03: return_moved_value
// Goal: Returning ownership
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string tag(std::string s){ std::cout<<"tagged\n"; return s; }
int main(){ std::cout<<tag("x")<<"\n"; }
CPP_EQUIVALENT_END */

fn tag(s:String)->String{ let t=s; println!("tagged"); s }
fn main(){ println!("{}",tag(String::from("x"))); }
