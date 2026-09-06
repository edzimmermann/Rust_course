// Topic: Option Result and Error Handling
// Solution 02: option_unwrap_or_type
// Fix: unwrap_or must receive the same owned type stored in the Option.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
int main(){std::optional<std::string> name;std::cout<<name.value_or("guest")<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let name:Option<String>=None;println!("{}",name.unwrap_or("guest".to_string()));}
