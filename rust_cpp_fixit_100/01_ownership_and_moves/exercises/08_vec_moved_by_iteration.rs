// Topic: Ownership and Moves
// Exercise 08: vec_moved_by_iteration
// Goal: for loop ownership
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){ std::vector<int> v{1,2,3}; for(int x:v) std::cout<<x<<"\n"; std::cout<<v.size()<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let v=vec![1,2,3]; for x in v { println!("{x}"); } println!("{}",v.len()); }
