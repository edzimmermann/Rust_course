// Topic: Traits and Generics
// Exercise 04: generic_max_copy
// Goal: Bounds for comparisons and returns
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
template<class T>T max2(T a,T b){return a>b?a:b;}
int main(){std::cout<<max2(3,7)<<"\n";}
CPP_EQUIVALENT_END */

fn max<T>(a:T,b:T)->T{if a>b{a}else{b}}
fn main(){println!("{}",max(3,7));}
