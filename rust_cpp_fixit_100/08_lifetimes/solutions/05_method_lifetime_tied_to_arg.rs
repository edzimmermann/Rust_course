// Topic: Lifetimes
// Solution 05: method_lifetime_tied_to_arg
// Fix: Explicitly tie the return lifetime to input, not to &self.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string_view>
struct Parser{std::string_view first(std::string_view s)const{auto p=s.find(',');return s.substr(0,p);}};
int main(){std::cout<<Parser{}.first("a,b")<<"\n";}
CPP_EQUIVALENT_END */

struct Parser;
impl Parser{fn first<'a>(&self,input:&'a str)->&'a str{input.split(',').next().unwrap()}}
fn main(){println!("{}",Parser.first("a,b"));}
