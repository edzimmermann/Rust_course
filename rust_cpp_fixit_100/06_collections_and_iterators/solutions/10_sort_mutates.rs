// Topic: Collections and Iterators
// Solution 10: sort_mutates
// Fix: sort mutates the collection and returns ().

/* CPP_EQUIVALENT_BEGIN
#include <algorithm>
#include <iostream>
#include <vector>
int main(){std::vector<int> v{3,1,2};std::sort(v.begin(),v.end());for(int x:v)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![3,1,2];v.sort();println!("{:?}",v);}
