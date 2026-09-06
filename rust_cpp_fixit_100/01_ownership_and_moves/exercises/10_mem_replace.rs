// Topic: Ownership and Moves
// Exercise 10: mem_replace
// Goal: Replacing an owned field
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
#include <utility>
struct Boxed{std::string value;};
std::string take(Boxed& b){ return std::exchange(b.value,{}); }
int main(){ Boxed b{"data"}; std::cout<<take(b)<<"\n"; }
CPP_EQUIVALENT_END */

struct Boxed{ value:String }
fn take(b:&mut Boxed)->String{ b.value }
fn main(){ let mut b=Boxed{value:"data".into()}; println!("{}",take(&mut b)); }
