// Topic: Collections and Iterators
// Exercise 10: sort_mutates
// Goal: sort works in place
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <vector>
int main(){std::vector<int> v{3,1,2};std::sort(v.begin(),v.end());for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let v=vec![3,1,2];let sorted=v.sort();println!("{:?}",sorted);}
