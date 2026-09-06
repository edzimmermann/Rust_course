// Topic: Option Result and Error Handling
// Exercise 05: ok_or_conversion
// Goal: Option to Result
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
#include <string_view>
std::optional<std::string_view> lookup(unsigned id){if(id==1)return "Ada";return std::nullopt;}
int main(){auto r=lookup(1);if(!r)return 1;std::cout<<*r<<"\n";}
CPP_EQUIVALENT_END */

fn lookup(id:u32)->Option<&'static str>{(id==1).then_some("Ada")}
fn main(){let r:Result<&str,&str>=lookup(1);println!("{}",r.unwrap());}
