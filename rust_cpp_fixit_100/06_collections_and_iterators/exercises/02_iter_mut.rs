// Topic: Collections and Iterators
// Exercise 02: iter_mut
// Goal: Mutating vector elements
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};for(int& x:v)x*=2;for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![1,2,3];for x in v.iter(){*x*=2;}println!("{:?}",v);}
