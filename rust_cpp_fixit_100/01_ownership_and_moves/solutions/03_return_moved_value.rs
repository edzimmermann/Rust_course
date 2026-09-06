// Topic: Ownership and Moves
// Solution 03: return_moved_value
// Fix: Return the owned parameter directly instead of trying to use it after moving it.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string tag(std::string s){ std::cout<<"tagged\n"; return s; }
int main(){ std::cout<<tag("x")<<"\n"; }
CPP_EQUIVALENT_END */

fn tag(s:String)->String{ println!("tagged"); s }
fn main(){ println!("{}",tag(String::from("x"))); }
