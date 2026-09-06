// Topic: Borrowing and References
// Solution 05: borrow_after_push
// Fix: Create the element reference after the vector mutation.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3}; v.push_back(4); const int& first=v[0]; std::cout<<first<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let mut v=vec![1,2,3]; v.push(4); let first=&v[0]; println!("{first}"); }
