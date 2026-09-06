// Topic: Lifetimes
// Exercise 05: method_lifetime_tied_to_arg
// Goal: Returning argument from a method
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
struct Parser{std::string_view first(std::string_view s)const{auto p=s.find(',');return s.substr(0,p);}};
int main(){std::cout<<Parser{}.first("a,b")<<"\n";}
CPP_EQUIVALENT_END */

struct Parser;
impl Parser{fn first(&self,input:&str)->&str{input.split(',').next().unwrap()}}
fn main(){println!("{}",Parser.first("a,b"));}
