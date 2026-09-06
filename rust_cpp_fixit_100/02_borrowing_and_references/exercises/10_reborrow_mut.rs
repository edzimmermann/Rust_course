// Topic: Borrowing and References
// Exercise 10: reborrow_mut
// Goal: Reborrowing a mutable reference
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
void bump(int& x){++x;}
int main(){int n=0; int& r=n; bump(r); bump(r); std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn bump(x:&mut i32){*x+=1;}
fn main(){let mut n=0;let r=&mut n;let r2=r;bump(r2);println!("{r}");}
