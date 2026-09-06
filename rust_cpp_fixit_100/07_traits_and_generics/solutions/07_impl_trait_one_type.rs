// Topic: Traits and Generics
// Solution 07: impl_trait_one_type
// Fix: impl Trait must resolve to one concrete type; use a trait object for heterogeneous branches.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <variant>
int main(){bool flag=true;std::variant<int,std::string> v=flag?std::variant<int,std::string>{42}:std::variant<int,std::string>{std::string("no")};std::visit([](const auto& x){std::cout<<x<<"\n";},v);}
CPP_EQUIVALENT_END */

use std::fmt::Display;
fn value(flag:bool)->Box<dyn Display>{if flag{Box::new(42)}else{Box::new("no")}}
fn main(){println!("{}",value(true));}
