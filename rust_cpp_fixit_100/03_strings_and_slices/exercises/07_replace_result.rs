// Topic: Strings and Slices
// Exercise 07: replace_result
// Goal: String methods return values
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <string>
int main(){std::string s="a-b-c";std::replace(s.begin(),s.end(),'-','_');std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut s=String::from("a-b-c");s.replace("-","_");println!("{s}");}
