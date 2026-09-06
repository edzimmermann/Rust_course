// Topic: Option Result and Error Handling
// Solution 04: map_vs_and_then
// Fix: Use and_then when the mapping function already returns an Option.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <optional>
std::optional<int> half(int n){if(n%2)return std::nullopt;return n/2;}
int main(){auto x=half(8);std::cout<<*x<<"\n";}
CPP_EQUIVALENT_END */

fn half(n:i32)->Option<i32>{(n%2==0).then_some(n/2)}
fn main(){let x=Some(8).and_then(half);println!("{}",x.unwrap());}
