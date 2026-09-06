// Topic: Traits and Generics
// Exercise 01: generic_display_bound
// Goal: Trait bounds on generics
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
template<class T> void show(const T& x){std::cout<<x<<"\n";}
int main(){show(42);}
CPP_EQUIVALENT_END */

fn show<T>(x:T){println!("{x}");}
fn main(){show(42);}
