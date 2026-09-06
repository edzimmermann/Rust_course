// Topic: Collections and Iterators
// Solution 01: collect_type
// Fix: Tell collect which collection type to build.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> squares;for(int x=1;x<=4;++x)squares.push_back(x*x);for(int x:squares)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let squares:Vec<i32>=(1..=4).map(|x|x*x).collect();println!("{:?}",squares);}
