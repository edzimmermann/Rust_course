// Topic: Borrowing and References
// Solution 10: reborrow_mut
// Fix: Explicitly reborrow with &mut *r for repeated calls.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
void bump(int& x){++x;}
int main(){int n=0; int& r=n; bump(r); bump(r); std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn bump(x:&mut i32){*x+=1;}
fn main(){let mut n=0;let r=&mut n;bump(&mut *r);bump(&mut *r);println!("{r}");}
