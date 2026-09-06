// Topic: Borrowing and References
// Solution 09: mutable_binding_needed
// Fix: Declare the owned binding mut before taking &mut.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="x"; std::string& r=s; r.push_back('y'); std::cout<<r<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let mut s=String::from("x"); let r=&mut s; r.push('y'); println!("{r}"); }
