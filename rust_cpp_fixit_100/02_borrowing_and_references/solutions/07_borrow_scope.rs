// Topic: Borrowing and References
// Solution 07: borrow_scope
// Fix: Stop using the immutable borrow before mutating the value.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){int n=1; const int& r=n; std::cout<<"now: "<<r<<"\n"; ++n; std::cout<<n<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let mut n=1; let r=&n; println!("now: {r}"); n+=1; println!("{n}"); }
