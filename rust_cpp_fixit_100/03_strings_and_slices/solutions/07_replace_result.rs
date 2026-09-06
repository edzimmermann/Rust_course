// Topic: Strings and Slices
// Solution 07: replace_result
// Fix: Use the String returned by replace; it does not mutate in place.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <string>
int main(){std::string s="a-b-c";std::replace(s.begin(),s.end(),'-','_');std::cout<<s<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s=String::from("a-b-c");let s=s.replace("-","_");println!("{s}");}
