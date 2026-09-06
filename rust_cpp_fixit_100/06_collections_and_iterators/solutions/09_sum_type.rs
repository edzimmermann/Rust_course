// Topic: Collections and Iterators
// Solution 09: sum_type
// Fix: Specify the accumulator/output type for sum when inference has no other clue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){unsigned total=0;for(unsigned x=1;x<=10;++x)total+=x;std::cout<<total<<"\n";}
CPP_EQUIVALENT_END */

fn main(){let total:u32=(1u8..=10).map(|x|x as u32).sum();println!("{total}");}
