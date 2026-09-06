// Topic: Option Result and Error Handling
// Exercise 06: option_as_ref
// Goal: Borrowing inside Option
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string>
int main(){std::optional<std::string> name="Ada";auto len=name->size();std::cout<<len<<" "<<*name<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let name=Some(String::from("Ada"));let len=name.map(|s|s.len());println!("{} {:?}",len.unwrap(),name);}
