// Topic: Strings and Slices
// Exercise 06: function_prefers_str
// Goal: Prefer &str parameters
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <cctype>
#include <iostream>
#include <string>
std::string shout(std::string s){for(char& c:s)c=static_cast<char>(std::toupper(static_cast<unsigned char>(c)));return s;}
int main(){std::cout<<shout("rust")<<"\n";}
CPP_EQUIVALENT_END */

fn shout(s:&String)->String{s.to_uppercase()}
fn main(){println!("{}",shout("rust"));}
