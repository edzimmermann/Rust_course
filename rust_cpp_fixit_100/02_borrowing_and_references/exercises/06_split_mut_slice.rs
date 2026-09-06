// Topic: Borrowing and References
// Exercise 06: split_mut_slice
// Goal: Disjoint mutable borrows
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3}; std::swap(v.front(),v.back()); for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn swap_edges(v:&mut [i32]){ let a=&mut v[0]; let b=&mut v[v.len()-1]; std::mem::swap(a,b); }
fn main(){let mut v=[1,2,3];swap_edges(&mut v);println!("{:?}",v);}
