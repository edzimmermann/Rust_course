// Topic: Lifetimes
// Exercise 07: longest_inner_scope
// Goal: Borrow cannot escape shorter scope
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string a="long",b="tiny";const std::string& r=a.size()>b.size()?a:b;std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let a=String::from("long");let r;{let b=String::from("tiny");r=if a.len()>b.len(){&a}else{&b};}println!("{r}");}
