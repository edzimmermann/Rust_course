// Topic: Collections and Iterators
// Exercise 07: vec_remove_index
// Goal: Vector remove uses usize index
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{10,20,30};std::size_t i=1;int x=v[i];v.erase(v.begin()+i);std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![10,20,30];let i:i32=1;println!("{}",v.remove(i));}
