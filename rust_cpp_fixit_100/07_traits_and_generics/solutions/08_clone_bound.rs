// Topic: Traits and Generics
// Solution 08: clone_bound
// Fix: Add a Clone bound before calling clone on a generic T.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
template<class T>std::pair<T,T> twice(T x){return {x,x};}
int main(){auto p=twice(std::string("x"));std::cout<<p.first<<p.second<<"\n";}
CPP_EQUIVALENT_END */

fn twice<T:Clone>(x:T)->(T,T){(x.clone(),x)}
fn main(){println!("{:?}",twice(String::from("x")));}
