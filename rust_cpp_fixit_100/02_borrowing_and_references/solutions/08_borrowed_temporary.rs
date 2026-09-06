// Topic: Borrowing and References
// Solution 08: borrowed_temporary
// Fix: Bind the owned temporary to a local before borrowing it.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
int main(){std::string s="temp"; const std::string& r=s; std::cout<<r<<"\n"<<r.size()<<"\n";}
CPP_EQUIVALENT_END */

fn main(){ let s=String::from("temp"); let r=&s; println!("{r}"); let n=r.len(); println!("{n}"); }
