// Topic: Borrowing and References
// Solution 06: split_mut_slice
// Fix: Use split_at_mut to prove the two mutable references are disjoint.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3}; std::swap(v.front(),v.back()); for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn swap_edges(v:&mut [i32]){ let n=v.len(); let (left,right)=v.split_at_mut(n-1); std::mem::swap(&mut left[0],&mut right[0]); }
fn main(){let mut v=[1,2,3];swap_edges(&mut v);println!("{:?}",v);}
