// Topic: Strings and Slices
// Solution 08: trim_result
// Fix: Use the &str returned by trim rather than assuming in-place mutation.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="  rust  ";auto first=s.find_first_not_of(' '),last=s.find_last_not_of(' ');std::cout<<"["<<s.substr(first,last-first+1)<<"]\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from("  rust  ");let trimmed=s.trim();println!("[{trimmed}]");}
