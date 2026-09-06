// Topic: Collections and Iterators
// Solution 02: iter_mut
// Fix: Use iter_mut to obtain mutable references.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};for(int& x:v)x*=2;for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![1,2,3];for x in v.iter_mut(){*x*=2;}println!("{:?}",v);}
