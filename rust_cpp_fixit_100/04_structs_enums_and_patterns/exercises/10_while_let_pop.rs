// Topic: Structs Enums and Patterns
// Exercise 10: while_let_pop
// Goal: while let patterns
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <vector>
int main(){std::vector<int> v{1,2,3};while(!v.empty()){int x=v.back();v.pop_back();std::cout<<x<<"\n";}}
CPP_EQUIVALENT_END */

fn main(){let mut v=vec![1,2,3];while let Some(x)=&v.pop(){println!("{x}");}}
