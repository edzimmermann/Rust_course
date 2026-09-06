// Topic: Collections and Iterators
// Solution 03: iterator_is_lazy
// Fix: Consume the iterator; for_each is appropriate for side effects.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};for(int x:v)std::cout<<x<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let v=vec![1,2,3];v.iter().for_each(|x|println!("{x}"));}
