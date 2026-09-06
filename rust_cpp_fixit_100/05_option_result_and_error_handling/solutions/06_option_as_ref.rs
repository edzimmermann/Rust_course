// Topic: Option Result and Error Handling
// Solution 06: option_as_ref
// Fix: Use as_ref before map to avoid consuming the Option<String>.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
int main(){std::optional<std::string> name="Ada";auto len=name->size();std::cout<<len<<" "<<*name<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let name=Some(String::from("Ada"));let len=name.as_ref().map(|s|s.len());println!("{} {:?}",len.unwrap(),name);}
