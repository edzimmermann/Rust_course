// Topic: Traits and Generics
// Solution 01: generic_display_bound
// Fix: Add the Display bound required by println!.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
template<class T> void show(const T& x){std::cout<<x<<"\n";}
int main(){show(42);}
CPP_EQUIVALENT_END */

fn show<T:std::fmt::Display>(x:T){println!("{x}");}
fn main(){show(42);}
