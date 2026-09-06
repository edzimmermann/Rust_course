// Topic: Traits and Generics
// Exercise 07: impl_trait_one_type
// Goal: impl Trait return consistency
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <variant>
int main(){bool flag=true;std::variant<int,std::string> v=flag?std::variant<int,std::string>{42}:std::variant<int,std::string>{std::string("no")};std::visit([](const auto& x){std::cout<<x<<"\n";},v);}
CPP_EQUIVALENT_END */

use std::fmt::Display;
fn value(flag:bool)->impl Display{if flag{42}else{"no"}}
fn main(){println!("{}",value(true));}
