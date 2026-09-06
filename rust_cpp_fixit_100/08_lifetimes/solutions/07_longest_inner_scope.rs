// Topic: Lifetimes
// Solution 07: longest_inner_scope
// Fix: Ensure every possible referent outlives the returned/used reference.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string a="long",b="tiny";const std::string& r=a.size()>b.size()?a:b;std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let a=String::from("long");let b=String::from("tiny");let r=if a.len()>b.len(){&a}else{&b};println!("{r}");}
