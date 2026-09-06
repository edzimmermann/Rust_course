// Topic: Smart Pointers and Interior Mutability
// Solution 02: rc_no_direct_mutation
// Fix: Use Rc::make_mut when uniquely mutating with clone-on-write semantics.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <memory>
#include <string>
int main(){auto x=std::make_shared<std::string>("a");x->push_back('b');std::cout<<*x<<"\n";}
CPP_EQUIVALENT_END */

use std::rc::Rc;
fn main(){let mut x=Rc::new(String::from("a"));Rc::make_mut(&mut x).push('b');println!("{x}");}
