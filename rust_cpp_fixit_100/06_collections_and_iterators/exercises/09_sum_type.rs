// Topic: Collections and Iterators
// Exercise 09: sum_type
// Goal: Iterator sum type inference
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){unsigned total=0;for(unsigned x=1;x<=10;++x)total+=x;std::cout<<total<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let total=(1u8..=10).map(|x|x as u32).sum();println!("{total}");}
