// Topic: Collections and Iterators
// Solution 08: enumerate_order
// Fix: Destructure enumerate items as (index, item).

/* CPP_EQUIVALENT_BEGIN
#include <cstddef>
#include <iostream>
#include <vector>
int main(){std::vector<const char*> v{"a","b"};for(std::size_t i=0;i<v.size();++i)std::cout<<i<<": "<<v[i]<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let v=["a","b"];for (index,item) in v.iter().enumerate(){println!("{index}: {item}");}}
