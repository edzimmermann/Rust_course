// Topic: Borrowing and References
// Exercise 03: borrow_parameter
// Goal: Borrow instead of owning
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::size_t length(const std::string& s){return s.size();}
int main(){std::string s="rust"; std::cout<<length(s)<<"\n"<<s<<"\n";}
CPP_EQUIVALENT_END */

fn length(s:String)->usize{s.len()}
fn main(){ let s=String::from("rust"); println!("{}",length(s)); println!("{s}"); }
