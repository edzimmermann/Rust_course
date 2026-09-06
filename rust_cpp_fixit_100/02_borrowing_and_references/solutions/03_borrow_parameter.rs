// Topic: Borrowing and References
// Solution 03: borrow_parameter
// Fix: Accept a borrowed string slice.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::size_t length(const std::string& s){return s.size();}
int main(){std::string s="rust"; std::cout<<length(s)<<"\n"<<s<<"\n";}
CPP_EQUIVALENT_END */

fn length(s:&str)->usize{s.len()}
fn main(){ let s=String::from("rust"); println!("{}",length(&s)); println!("{s}"); }
