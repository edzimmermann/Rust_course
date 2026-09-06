// Topic: Borrowing and References
// Exercise 07: borrow_scope
// Goal: Non-lexical lifetimes
// The Rust version below has one small issue.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
int main(){int n=1; const int& r=n; std::cout<<"now: "<<r<<"\n"; ++n; std::cout<<n<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let mut n=1; let r=&n; println!("later: {r}"); n+=1; println!("{r} {n}"); }
