// Topic: Option Result and Error Handling
// Exercise 03: result_unwrap_or_else
// Goal: Result fallback closure argument
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <variant>
int main(){std::variant<int,const char*> r="bad";int n=std::holds_alternative<int>(r)?std::get<int>(r):0;std::cout<<n<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let r:Result<i32,&str>=Err("bad");let n=r.unwrap_or_else(||0);println!("{n}");}
