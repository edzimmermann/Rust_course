// Topic: Collections and Iterators
// Solution 04: filter_reference_level
// Fix: filter receives a reference to each iterator item, so dereference one extra level.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3,4};for(int x:v)if(x%2==0)std::cout<<x<<" ";}
CPP_EQUIVALENT_END */

fn main(){let v=vec![1,2,3,4];let evens:Vec<&i32>=v.iter().filter(|x|**x%2==0).collect();println!("{:?}",evens);}
