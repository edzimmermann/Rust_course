// Topic: Ownership and Moves
// Solution 04: copy_integer_expectation
// Fix: Remove the meaningless drop of a Copy value; integers remain usable after assignment.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){ int n=7; int m=n; std::cout<<n<<" "<<m<<"\n"; }
CPP_EQUIVALENT_END */

fn main(){ let n=7; let m=n; println!("{n} {m}"); }
