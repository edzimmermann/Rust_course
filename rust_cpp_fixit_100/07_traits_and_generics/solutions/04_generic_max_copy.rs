// Topic: Traits and Generics
// Solution 04: generic_max_copy
// Fix: Add PartialOrd because the function uses >.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
template<class T>T max2(T a,T b){return a>b?a:b;}
int main(){std::cout<<max2(3,7)<<"\n";}
CPP_EQUIVALENT_END */

fn max<T:PartialOrd>(a:T,b:T)->T{if a>b{a}else{b}}
fn main(){println!("{}",max(3,7));}
