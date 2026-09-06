// Topic: Collections and Iterators
// Solution 07: vec_remove_index
// Fix: Collection indices use usize.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{10,20,30};std::size_t i=1;int x=v[i];v.erase(v.begin()+i);std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![10,20,30];let i:usize=1;println!("{}",v.remove(i));}
