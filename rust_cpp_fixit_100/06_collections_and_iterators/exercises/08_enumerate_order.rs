// Topic: Collections and Iterators
// Exercise 08: enumerate_order
// Goal: enumerate returns index then item
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <cstddef>
#include <iostream>
#include <vector>
int main(){std::vector<const char*> v{"a","b"};for(std::size_t i=0;i<v.size();++i)std::cout<<i<<": "<<v[i]<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let v=["a","b"];for (item,index) in v.iter().enumerate(){println!("{index}: {item}");}}
