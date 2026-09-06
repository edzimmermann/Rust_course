// Topic: Strings and Slices
// Exercise 08: trim_result
// Goal: Borrowed string transformations
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="  rust  ";auto first=s.find_first_not_of(' '),last=s.find_last_not_of(' ');std::cout<<"["<<s.substr(first,last-first+1)<<"]\n";}
CPP_EQUIVALENT_END */

fn main(){let mut s=String::from("  rust  ");s.trim();println!("[{s}]");}
