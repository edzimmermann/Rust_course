// Topic: Lifetimes
// Exercise 01: dangling_reference
// Goal: References cannot outlive owners
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string make(){return "hello";}
int main(){std::cout<<make()<<"\n";}
CPP_EQUIVALENT_END */

fn make()->&'static str{let s=String::from("hello");&s}
fn main(){println!("{}",make());}
