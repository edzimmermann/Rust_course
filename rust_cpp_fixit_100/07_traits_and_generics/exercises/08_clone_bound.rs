// Topic: Traits and Generics
// Exercise 08: clone_bound
// Goal: Calling clone in generic code
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
template<class T>std::pair<T,T> twice(T x){return {x,x};}
int main(){auto p=twice(std::string("x"));std::cout<<p.first<<p.second<<"\n";}
CPP_EQUIVALENT_END */

fn twice<T>(x:T)->(T,T){(x.clone(),x)}
fn main(){println!("{:?}",twice(String::from("x")));}
