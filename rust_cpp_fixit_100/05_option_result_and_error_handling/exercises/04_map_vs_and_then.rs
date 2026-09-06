// Topic: Option Result and Error Handling
// Exercise 04: map_vs_and_then
// Goal: Flattening Option
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
std::optional<int> half(int n){if(n%2)return std::nullopt;return n/2;}
int main(){auto x=half(8);std::cout<<*x<<"\n";}
CPP_EQUIVALENT_END */

fn half(n:i32)->Option<i32>{(n%2==0).then_some(n/2)}
fn main(){let x=Some(8).map(half);println!("{}",x.unwrap());}
