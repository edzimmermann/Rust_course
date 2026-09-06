// Topic: Collections and Iterators
// Exercise 03: iterator_is_lazy
// Goal: Iterator adapters are lazy
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};for(int x:v)std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let v=vec![1,2,3];v.iter().map(|x|println!("{x}"));}
