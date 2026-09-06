// Topic: Option Result and Error Handling
// Exercise 02: option_unwrap_or_type
// Goal: Option default type
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
int main(){std::optional<std::string> name;std::cout<<name.value_or("guest")<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let name:Option<String>=None;println!("{}",name.unwrap_or("guest"));}
