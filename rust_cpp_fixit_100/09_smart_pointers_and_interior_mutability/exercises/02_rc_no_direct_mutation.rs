// Topic: Smart Pointers and Interior Mutability
// Exercise 02: rc_no_direct_mutation
// Goal: Rc does not provide shared mutation
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto x=std::make_shared<std::string>("a");x->push_back('b');std::cout<<*x<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let x=Rc::new(String::from("a"));x.push('b');println!("{x}");}
