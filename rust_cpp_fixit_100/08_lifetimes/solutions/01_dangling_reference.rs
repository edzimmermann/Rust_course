// Topic: Lifetimes
// Solution 01: dangling_reference
// Fix: Return owned data instead of a reference to a local String.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string make(){return "hello";}
int main(){std::cout<<make()<<"\n";}
CPP_EQUIVALENT_END */

fn make()->String{String::from("hello")}
fn main(){println!("{}",make());}
