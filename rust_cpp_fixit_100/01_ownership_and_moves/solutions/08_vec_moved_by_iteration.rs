// Topic: Ownership and Moves
// Solution 08: vec_moved_by_iteration
// Fix: Iterate over &v so the vector is borrowed rather than consumed.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){ std::vector<int> v{1,2,3}; for(int x:v) std::cout<<x<<"\n"; std::cout<<v.size()<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let v=vec![1,2,3]; for x in &v { println!("{x}"); } println!("{}",v.len()); }
