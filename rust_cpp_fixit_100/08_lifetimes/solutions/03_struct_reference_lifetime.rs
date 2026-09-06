// Topic: Lifetimes
// Solution 03: struct_reference_lifetime
// Fix: Add a lifetime parameter to a struct that stores a reference.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <string_view>
struct View{std::string_view text;};
int main(){std::string s="abc";View v{s};std::cout<<v.text<<"\n";}
CPP_EQUIVALENT_END */

struct View<'a>{ text:&'a str }
fn main(){let s=String::from("abc");let v=View{text:&s};println!("{}",v.text);}
