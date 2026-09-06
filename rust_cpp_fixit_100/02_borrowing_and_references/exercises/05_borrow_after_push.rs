// Topic: Borrowing and References
// Exercise 05: borrow_after_push
// Goal: Vector reallocation and references
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3}; v.push_back(4); const int& first=v[0]; std::cout<<first<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let mut v=vec![1,2,3]; let first=&v[0]; v.push(4); println!("{first}"); }
