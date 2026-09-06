// Topic: Structs Enums and Patterns
// Solution 10: while_let_pop
// Fix: Match the Option value directly rather than a reference to it.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};while(!v.empty()){int x=v.back();v.pop_back();std::cout<<x<<"\n";}}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![1,2,3];while let Some(x)=v.pop(){println!("{x}");}}
