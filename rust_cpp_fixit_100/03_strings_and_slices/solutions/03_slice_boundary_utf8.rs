// Topic: Strings and Slices
// Solution 03: slice_boundary_utf8
// Fix: Slice at a UTF-8 character boundary; é occupies two bytes.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="\xC3\xA9" "clair";std::cout<<s.substr(0,2)<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let s="éclair";println!("{}",&s[0..2]);}
