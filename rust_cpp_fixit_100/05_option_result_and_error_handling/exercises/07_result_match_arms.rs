// Topic: Option Result and Error Handling
// Exercise 07: result_match_arms
// Goal: Handling both Result variants
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <variant>
int main(){std::variant<int,const char*> r=3;if(std::holds_alternative<int>(r))std::cout<<std::get<int>(r)<<"\n";else std::cerr<<std::get<const char*>(r)<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let r:Result<i32,&str>=Ok(3);match r{Ok(n)=>println!("{n}")}}
