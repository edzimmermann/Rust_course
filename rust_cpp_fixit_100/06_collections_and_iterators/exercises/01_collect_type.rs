// Topic: Collections and Iterators
// Exercise 01: collect_type
// Goal: Iterator collect target type
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> squares;for(int x=1;x<=4;++x)squares.push_back(x*x);for(int x:squares)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let squares=(1..=4).map(|x|x*x).collect();println!("{:?}",squares);}
