// Topic: Lifetimes
// Solution 09: return_local_slice
// Fix: Return an owned String when slicing a local owner.

/* CPP_EQUIVALENT_BEGIN
#include <iostream>
#include <string>
std::string prefix(){std::string s="abcdef";return s.substr(0,3);}
int main(){std::cout<<prefix()<<"\n";}
CPP_EQUIVALENT_END */

fn prefix()->String{let s=String::from("abcdef");s[..3].to_string()}
fn main(){println!("{}",prefix());}
